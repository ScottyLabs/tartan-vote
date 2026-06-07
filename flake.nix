{
  description = "Tartan Vote";

  nixConfig = {
    extra-substituters = [ "https://scottylabs.cachix.org" ];
    extra-trusted-public-keys = [
      "scottylabs.cachix.org-1:hajjEX5SLi/Y7yYloiXTt2IOr3towcTGRhMh1vu6Tjg="
    ];
  };

  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    crane.url = "github:ipetkov/crane";
    devenv.url = "github:cachix/devenv";
    scottylabs.url = "git+https://codeberg.org/ScottyLabs/devenv";
    bun2nix = {
      url = "github:nix-community/bun2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, devenv, scottylabs, bun2nix, rust-overlay, ... } @ inputs:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ];
      pkgsFor = system: import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      mkFrontend =
        { apiBase, authBase }:
        pkgs: pkgs.stdenv.mkDerivation {
          pname = "tartan-vote-frontend";
          version = "0.1.0";
          src = ./frontend;
          nativeBuildInputs = [ pkgs.deno ];
          buildPhase = ''
            export DENO_DIR="$TMPDIR/deno"
            export HOME="$TMPDIR"
            deno install --allow-scripts
            export VITE_API_BASE="${apiBase}"
            export VITE_BETTER_AUTH_BASE_URL="${authBase}"
            export VITE_BETTER_AUTH_PROVIDER_ID="cmu-sso"
            deno run build
          '';
          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
        };
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = pkgsFor system;
        in
        {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [ ./devenv.nix ];
          };
        });

      packages = forAllSystems (system:
        let
          pkgs = pkgsFor system;
          rustToolchain = pkgs.rust-bin.stable.latest.default;
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          commonArgs = {
            pname = "api";
            version = "0.1.0";
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          auth = pkgs.buildNpmPackage {
            pname = "tartan-vote-auth";
            version = "0.1.0";
            src = ./auth-service;
            npmDepsHash = "sha256-/7XqvJAGyGSE3SudNDK0/vlOKUqif4nqfW5WN5mMsLs=";
            npmFlags = [ "--omit=dev" ];
            dontNpmBuild = true;
            nativeBuildInputs = [ pkgs.makeWrapper ];
            postInstall = ''
              mkdir -p $out/bin
              makeWrapper ${pkgs.nodejs_22}/bin/node $out/bin/auth \
                --chdir $out \
                --add-flags server.mjs
            '';
          };

          frontend = mkFrontend {
            apiBase = "https://api.tartan-vote.scottylabs.org";
            authBase = "https://auth.tartan-vote.scottylabs.org/api/auth";
          } pkgs;
        in
        {
          devenv = devenv.packages.${system}.devenv;

          api = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
            doCheck = false;
          });

          inherit auth frontend;

          default = self.packages.${system}.api;
        }
      );
    };
}

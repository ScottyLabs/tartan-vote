{
  description = "tartan-vote deployment packages";

  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    scottylabs = {
      url = "git+https://codeberg.org/ScottyLabs/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, nixpkgs, scottylabs, ... }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ];
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          lib = pkgs.lib;
          helpers = scottylabs.mkLib pkgs;

          cleanJsSrc =
            path:
            lib.cleanSourceWith {
              src = path;
              filter =
                p: _t:
                let
                  base = baseNameOf p;
                in
                  !(builtins.elem base [
                    "node_modules"
                    "package-lock.json"
                    "dist"
                  ]);
            };
        in
        {
          api = helpers.buildRustService {
            src = ./.;
            pname = "api";
            buildInputs = [ pkgs.openssl ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildArgs.cargoExtraArgs = "-p backend";
          };

          frontend = (helpers.buildDenoTask {
            src = cleanJsSrc ./frontend;
            pname = "frontend";
          });

          default = self.packages.${system}.api;
        }
      );
    };
}

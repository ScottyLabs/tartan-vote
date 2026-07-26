{
  description = "Tartan.vote";

  nixConfig = {
    extra-substituters = [ "https://scottylabs.cachix.org" ];
    extra-trusted-public-keys = [
      "scottylabs.cachix.org-1:hajjEX5SLi/Y7yYloiXTt2IOr3towcTGRhMh1vu6Tjg="
    ];
  };

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    scottylabs = {
      url = "git+https://codeberg.org/ScottyLabs/kennel";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      scottylabs,
      ...
    }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          helpers = scottylabs.mkLib pkgs;

          frontend = helpers.buildDenoTask {
            src = ./frontend;
            pname = "frontend";
            task = "build";
          };

          tartan-vote = helpers.buildRustService {
            src = ./.;
            pname = "tartan-vote";
            buildInputs = [ pkgs.openssl ];
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.makeWrapper
            ];
            buildArgs = {
              cargoExtraArgs = "-p backend";
              postInstall = ''
                mkdir -p $out/share/tartan-vote/www
                cp -r ${frontend}/* $out/share/tartan-vote/www/
                chmod -R u+w $out/share/tartan-vote/www
                wrapProgram $out/bin/backend \
                  --set-default STATIC_DIR $out/share/tartan-vote/www
              '';
            };
          };
        in
        {
          inherit frontend tartan-vote;
        }
      );
    };
}

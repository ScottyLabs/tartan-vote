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
    {
      self,
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
        in
        {
          inherit frontend;

          tartan-vote = helpers.buildRustService {
            src = ./.;
            pname = "tartan-vote";
            buildInputs = [ pkgs.openssl ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildArgs.cargoExtraArgs = "-p backend";
          };

          default = self.packages.${system}.tartan-vote;
        }
      );
    };
}

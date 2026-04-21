{ pkgs, inputs, ... }:

{
  imports = [ inputs.scottylabs.devenvModules.default ];

  scottylabs = {
    enable = true;
    project.name = "voting-app";
    rust.enable = true;
    bun.enable = true;
    postgres.enable = true;
  };

  packages = [
    inputs.bun2nix.packages.${pkgs.stdenv.system}.default
  ];
}

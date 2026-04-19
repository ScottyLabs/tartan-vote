{ pkgs, lib, config, inputs, ... }:

{
  imports = [ inputs.scottylabs.devenvModules.default ];

  scottylabs = {
    enable = true;
    project.name = "voting-app";
    rust.enable = true;
    bun.enable = true;
  };

  packages = [
    inputs.bun2nix.packages.${pkgs.stdenv.system}.default
  ] ++ (with pkgs; [
    bun
    crate2nix
  ]);

  git-hooks.hooks = {
    clippy.settings.extraArgs = "--manifest-path backend/Cargo.toml";
    oxlint = lib.mkForce {
      enable = true;
      entry = "${pkgs.oxlint}/bin/oxlint --fix";
      files = "\\.(js|ts|mjs|mts|cjs|cts|jsx|tsx)$";
      language = "system";
      pass_filenames = true;
    };
    cargo-nix-update = lib.mkForce {
      enable = true;
      entry = "${pkgs.writeShellScript "cargo-nix-update" ''
        if git diff --cached --name-only | grep -qE '^backend/Cargo\.(toml|lock)$'; then
          cd ${lib.escapeShellArg config.devenv.root}/backend
          ${pkgs.crate2nix}/bin/crate2nix generate
          git add Cargo.nix
        fi
      ''}";
      files = "^backend/Cargo\\.(toml|lock)$";
      language = "system";
      pass_filenames = false;
    };
  };
}

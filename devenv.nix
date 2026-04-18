{ pkgs, lib, config, inputs, ... }:

{
  cachix.pull = [ "scottylabs" ];

  packages = [
    inputs.bun2nix.packages.${pkgs.stdenv.system}.default
  ] ++ (with pkgs; [
    bun
    crate2nix
  ]);

  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "rust-src"
    ];
  };

  dotenv.enable = true;

  treefmt = {
    enable = true;
    # Paths are relative to repo root; globs supported. Root-only "Cargo.nix" / "bun.nix" do not match subdirs.
    config.settings.excludes = lib.mkAfter [
      "backend/Cargo.nix"
      "frontend/bun.nix"
    ];
    config.programs = {
      nixpkgs-fmt = {
        enable = true;
        excludes = [ "Cargo.nix" "bun.nix" ];
      };
      rustfmt.enable = true;
    };
    config.settings.formatter.biome = {
      command = "${pkgs.biome}/bin/biome";
      options = [ "check" "--write" "--no-errors-on-unmatched" "--config-path" "${config.devenv.root}/biome.json" ];
      includes = [ "*.js" "*.ts" "*.mjs" "*.mts" "*.cjs" "*.cts" "*.jsx" "*.tsx" "*.d.ts" "*.d.cts" "*.d.mts" "*.json" "*.jsonc" "*.css" ];
    };
  };

  git-hooks.hooks = {
    treefmt = {
      enable = true;
      # pre-commit passes staged paths as CLI args; treefmt formats them anyway, so
      # treefmt.toml `excludes` alone does not help — skip these files at the hook level.
      excludes = [
        "^backend/Cargo\\.nix$"
        "^frontend/bun\\.nix$"
      ];
    };
    clippy = {
      enable = true;
      packageOverrides.cargo = config.languages.rust.toolchainPackage;
      packageOverrides.clippy = config.languages.rust.toolchainPackage;
      settings.allFeatures = true;
      settings.denyWarnings = true;
      settings.extraArgs = "--manifest-path backend/Cargo.toml";
    };
    cargo-nix-update = {
      enable = true;
      name = "crate2nix (backend/Cargo.nix)";
      description = "Regenerate backend/Cargo.nix when backend/Cargo.toml or Cargo.lock are staged";
      entry = "${pkgs.writeShellScript "cargo-nix-update" ''
        set -euo pipefail
        root=${lib.escapeShellArg config.devenv.root}
        if git diff --cached --name-only | grep -qE '^backend/Cargo\.(toml|lock)$'; then
          (cd "''${root}/backend" && ${pkgs.crate2nix}/bin/crate2nix generate)
          git add "''${root}/backend/Cargo.nix"
        fi
      ''}";
      files = "^backend/Cargo\\.(toml|lock)$";
      language = "system";
      pass_filenames = false;
      extraPackages = [ config.languages.rust.toolchainPackage ];
    };
  };
}

{
  lib,
  pkgs,
  inputs,
  ...
}:

let
  # .env.shared is the single source of truth for the port.
  port = lib.removePrefix "PORT=" (lib.removeSuffix "\n" (builtins.readFile ./.env.shared));
in
{
  imports = [ inputs.scottylabs.devenvModules.default ];

  scottylabs = {
    enable = true;
    project.name = "tartan-vote";

    conventionalCommits.enable = false;

    # temp fixes
    security.semgrep.enable = false;

    rust.enable = true;
    deno = {
      enable = true;
      dirs = [ "frontend" ];
    };
    postgres.enable = true;
    valkey.enable = true;

    ricochet = {
      enable = true;
      appUrl = "http://localhost:${port}";
    };
    kennel = {
      services.tartan-vote.customDomain = "tartan.vote";
    };
  };

  dotenv.enable = true;
  dotenv.filename = ".env.shared";

  # more temp fixes
  languages.rust.lld.enable = lib.mkIf pkgs.stdenv.isDarwin (lib.mkForce false);
  languages.rust.cranelift.enable = lib.mkIf pkgs.stdenv.isDarwin (lib.mkForce false);

  packages = [ pkgs.git ];

  # Built SPA served by the backend (run `deno task build` in frontend/ first).
  env.STATIC_DIR = "frontend/dist";

  git-hooks.hooks = {
    deno-check.entry = lib.mkForce "bash -c 'cd frontend && deno check .'";
    deno-test.entry = lib.mkForce "deno test --ignore=.devenv,.direnv --permit-no-files";
    block-ai-slop.excludes = [ "^docs/" ];
    oxlint.settings.deny = lib.mkForce [
      "correctness"
      "suspicious"
      "perf"
    ];
  };

  scripts = {
    generate-api.exec = "cd frontend && deno task generate-api";
  };
}

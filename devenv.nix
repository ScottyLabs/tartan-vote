{
  lib,
  pkgs,
  inputs,
  ...
}:

{
  imports = [ inputs.scottylabs.devenvModules.default ];

  scottylabs = {
    enable = true;
    project.name = "tartan-vote";

    conventionalCommits.enable = false;

    # Semgrep 1.180.0 fails in patchelf on ARM Linux; 1.164.0 also
    # segfaults at runtime. Re-enable after verifying upstream ARM support.
    security.semgrep.enable = pkgs.stdenv.hostPlatform.system != "aarch64-linux";

    rust.enable = true;
    deno = {
      enable = true;
      dirs = [ "frontend" ];
    };
    postgres.enable = true;
    valkey.enable = true;

    ricochet = {
      enable = true;
      appUrl = "http://localhost:8080";
    };
    kennel = {
      services.tartan-vote.customDomain = "tartan.vote";
    };
  };

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

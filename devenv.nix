{
  lib,
  inputs,
  ...
}:

{
  imports = [ inputs.scottylabs.devenvModules.default ];

  scottylabs = {
    enable = true;
    project.name = "tartan-vote";

    conventionalCommits.enable = false;

    rust.enable = true;
    deno.enable = true;
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

  # Built SPA served by the backend (run `deno task build` in frontend/ first).
  env.STATIC_DIR = "frontend/dist";

  git-hooks.hooks = {
    deno-check.entry = lib.mkForce "bash -c 'cd frontend && deno check .'";
    deno-test.entry = lib.mkForce "deno test --ignore=.devenv,.direnv --permit-no-files";
  };

  scripts = {
    generate-api.exec = "cd frontend && deno task generate-api";
  };
}

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

    ricochet = {
      enable = true;
      appUrl = "http://localhost:8080";
    };
    kennel = {
      sites.frontend = {
        spa = true;
        customDomain = "tartan.vote";
      };
      services.tartan-vote.customDomain = "api.tartan.vote";
    };
  };

  git-hooks.hooks = {
    deno-check.entry = lib.mkForce ''
      bash -c 'cd frontend && args=(); for f in "$@"; do args+=("''${f#frontend/}"); done; deno check "''${args[@]}"' --
    '';
    deno-test.entry = lib.mkForce "deno test --ignore=.devenv,.direnv,old-frontend --permit-no-files";
  };

  git-hooks.excludes = [ "old-frontend/.*" ];
  treefmt.config.settings.excludes = [ "old-frontend/**" ];

  scripts = {
    generate-api.exec = "cd frontend && deno task generate-api";
  };
}

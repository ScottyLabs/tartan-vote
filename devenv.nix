{ config, pkgs, lib, inputs, ... }:

{
  imports = [ inputs.scottylabs.devenvModules.default ];

  scottylabs = {
    enable = true;
    project.name = "tartan-vote";

    conventionalCommits.enable = false;

    rust.enable = true;
    deno.enable = true;
    postgres.enable = true;

    ricochet.enable = true;
    ricochet.appUrl = "http://localhost:8080";

    kennel.services.api = {
      customDomain = "tartan-vote.scottylabs.org";
      oidc.redirectPaths = [ "/auth/callback" ];
    };
  };

  cachix.enable = false;

  # The ScottyLabs deno module runs `oxlint --deny all`, which force-enables
  # every opt-in rule -- including contradictory restriction/style rules that
  # ban async/await, ternaries, null, magic numbers, etc. Allow those categories
  # so only the meaningful ones (correctness, suspicious, perf) stay enforced.
  git-hooks.hooks.oxlint.settings.allow = [
    "restriction"
    "style"
    "pedantic"
    "nursery"
  ];

  processes = {
    api.exec = ''
      set -euo pipefail
      (cd frontend && deno install && deno task build)
      exec secretspec run --profile dev -- cargo run
    '';
    frontend-watch = {
      exec = "deno install && deno task build:watch";
      cwd = "./frontend";
    };
  };

  env = {
    VAULT_ADDR = "https://secrets2.scottylabs.org";
    SECRETSPEC_PROFILE = "dev";
    SECRETSPEC_PROVIDER = "vault://secrets2.scottylabs.org/secret";
  };
}

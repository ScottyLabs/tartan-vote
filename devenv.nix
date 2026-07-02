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

    kennel.services.tartan-vote = {
      customDomain = "tartan-vote.scottylabs.org";
      oidc.redirectPaths = [ "/auth/callback" ];
    };
  };

  cachix.enable = false;

  env = {
    VAULT_ADDR = "https://secrets2.scottylabs.org";
    SECRETSPEC_PROFILE = "dev";
    SECRETSPEC_PROVIDER = "vault://secrets2.scottylabs.org/secret";
  };
}

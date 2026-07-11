{
  config,
  pkgs,
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

    ricochet.enable = true;
    ricochet.appUrl = "http://localhost:8080";

    kennel.services.tartan-vote = {
      customDomain = "api.tartan.vote";
    };

    kennel.sites.frontend = {
      spa = true;
      customDomain = "tartan.vote";
    };
  };

  cachix.enable = false;

  scripts = {
    generate-api.exec = "cd frontend && deno task generate-api";
  };

  env = {
    VAULT_ADDR = "https://secrets2.scottylabs.org";
    SECRETSPEC_PROFILE = "dev";
    SECRETSPEC_PROVIDER = "vault://secrets2.scottylabs.org/secret";
  };
}

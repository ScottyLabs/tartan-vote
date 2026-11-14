{
  description = "Tartan.vote";

  nixConfig = {
    extra-substituters = [ "https://scottylabs.cachix.org" ];
    extra-trusted-public-keys = [
      "scottylabs.cachix.org-1:hajjEX5SLi/Y7yYloiXTt2IOr3towcTGRhMh1vu6Tjg="
    ];
  };

  inputs = {
    scottylabs.url = "git+https://git.cmu.dev/ScottyLabs/kennel";
    nixpkgs.follows = "scottylabs/nixpkgs";
    flake-parts.follows = "scottylabs/flake-parts";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.scottylabs.flakeModules.default ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        {
          lib,
          pkgs,
          helpers,
          ...
        }:
        let
          frontend = helpers.buildDenoTask {
            src = ./frontend;
            pname = "tartan-vote-frontend";
          };
        in
        {
          scottylabs = {
            project.name = "tartan-vote";
            conventionalCommits.enable = false;

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

            processes = {
              backend = {
                run = "exec secretspec run -P dev -- cargo run -p backend";
                depends = [
                  "postgres-setup"
                  "valkey"
                ];
              };
              frontend.run = "cd frontend && deno task build:watch";
            };

            shell = {
              packages = [ pkgs.sea-orm-cli ];
              # the backend serves the SPA that frontend.run builds
              env.STATIC_DIR = "frontend/dist";
              scripts.generate-api = "cd frontend && deno task generate-api";
            };
          };

          pre-commit.settings.hooks = {
            deno-check.entry = lib.mkForce "bash -c 'cd frontend && deno check .'";
            deno-test.entry = lib.mkForce "deno test --ignore=.scottylabs --permit-no-files";
            block-ai-slop.excludes = [ "^docs/" ];
            oxlint.settings.deny = lib.mkForce [
              "correctness"
              "suspicious"
              "perf"
            ];
          };

          services.tartan-vote = {
            package = helpers.buildRustService {
              src = ./.;
              pname = "tartan-vote";
              buildArgs.cargoExtraArgs = "--locked -p backend";
            };
            customDomain = "tartan.vote";
            env.STATIC_DIR = "${frontend}";
          };
        };
    };
}

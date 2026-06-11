{ config, pkgs, lib, inputs, ... }:

let
  denoPackages = pkgs.callPackage ./nix/deno-package.nix { };

  cleanJsSrc = path:
    lib.cleanSourceWith {
      src = path;
      filter = p: _t:
        let base = baseNameOf p;
        in !(builtins.elem base [
          "node_modules"
          "package-lock.json"
          "dist"
        ]);
    };

  # Pin on x86_64-linux after the first Kennel build prints "got: sha256-...".
  denoDepsHash = lib.fakeHash;
in
{
  imports = [ inputs.scottylabs.devenvModules.default ];

  scottylabs = {
    enable = true;
    project.name = "tartan-vote";

    rust.enable = true;
    deno.enable = true;
    postgres.enable = true;

    kennel.services.api = {
      customDomain = "api.tartan-vote.scottylabs.org";
      oidc.redirectPaths = [ "/auth/callback" ];
    };
    kennel.services.auth = {
      customDomain = "auth.tartan-vote.scottylabs.org";
    };
    kennel.sites.frontend = {
      spa = true;
      customDomain = "tartan-vote.scottylabs.org";
    };
  };

  cachix.enable = false;

  outputs = {
    api = config.languages.rust.import ./. {
      buildInputs = [ pkgs.openssl ];
      nativeBuildInputs = [ pkgs.pkg-config ];
    };

    auth = denoPackages.mkDenoNodeService {
      pname = "auth";
      src = cleanJsSrc ./auth-service;
      command = "server.mjs";
      depsHash = denoDepsHash;
    };

    frontend = denoPackages.mkDenoViteFrontend {
      pname = "frontend";
      src = cleanJsSrc ./frontend;
      apiBase = "https://api.tartan-vote.scottylabs.org";
      authBase = "https://auth.tartan-vote.scottylabs.org/api/auth";
      depsHash = denoDepsHash;
    };
  };

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
    api.exec = "secretspec run --profile dev -- cargo run";
    # auth = {
    #   exec = "secretspec run --profile dev -- node server.mjs";
    #   cwd = "./auth-service";
    # };
    frontend = {
      exec = "deno install && deno run dev --host";
      cwd = "./frontend";
    };
  };

  # Make DATABASE_URL follow whatever port the running devenv Postgres actually
  # bound. devenv exports a portless DATABASE_URL / PGPORT=5432, but when the
  # host already runs a Postgres on 5432 the managed server lands on 5433. The
  # live port is in postmaster.pid (line 4); fall back to PGPORT, then 5432.
  enterShell = ''
    # Host-specific URLs are derived from a single DEV_HOST so the shared secret
    # store never carries a developer's machine address. For cross-machine
    # testing (serve here, browse from another device) set DEV_HOST to this
    # machine's LAN IP in a git-ignored .env.local, e.g. DEV_HOST=192.168.1.20,
    # and register http://$DEV_HOST:8080/auth/callback on the Keycloak client.
    if [ -r "$PWD/.env.local" ]; then
      set -a; . "$PWD/.env.local"; set +a
    fi
    DEV_HOST="''${DEV_HOST:-localhost}"
    export DEV_HOST
    export APP_BASE_URL="http://$DEV_HOST:8080"
    export FRONTEND_BASE_URL="http://$DEV_HOST:5173"
    export BETTER_AUTH_URL="http://$DEV_HOST:3005"
    export BETTER_AUTH_BASE_URL="http://$DEV_HOST:3005/api/auth"
    export VITE_API_BASE="http://$DEV_HOST:8080"
    export VITE_BETTER_AUTH_BASE_URL="http://$DEV_HOST:3005/api/auth"
    export CORS_ALLOWED_ORIGINS="http://$DEV_HOST:5173,http://$DEV_HOST:8080,http://$DEV_HOST:3005"

    pg_port="''${PGPORT:-5432}"
    if [ -n "''${PGDATA:-}" ] && [ -f "''${PGDATA}/postmaster.pid" ]; then
      pg_port="$(sed -n '4p' "''${PGDATA}/postmaster.pid" 2>/dev/null || echo "$pg_port")"
    fi
    if [ -n "''${DATABASE_URL:-}" ]; then
      export DATABASE_URL="$(printf '%s' "$DATABASE_URL" | sed -E "s/(:[0-9]+)(\/|$)/:$pg_port\\2")"
    fi
  '';

  env = {
    VAULT_ADDR = "https://secrets2.scottylabs.org";
    SECRETSPEC_PROFILE = "dev";
    SECRETSPEC_PROVIDER = "vault://secrets2.scottylabs.org/secret";

    # Non-secret, machine-independent constants. Host-specific URLs are derived
    # from DEV_HOST in enterShell above.
    OIDC_ISSUER = "https://idp.scottylabs.org/realms/scottylabs";
    BETTER_AUTH_PROVIDER_ID = "cmu-sso";
    VITE_BETTER_AUTH_PROVIDER_ID = "cmu-sso";
    BETTER_AUTH_PORT = "3005";
    BIND_ADDR = "0.0.0.0:8080";
  };
}

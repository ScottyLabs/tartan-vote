{ config, pkgs, inputs, ... }:

{
  imports = [ inputs.scottylabs.devenvModules.default ];

  scottylabs = {
    enable = true;
    project.name = "tartan-vote";

    rust.enable = true;
    deno.enable = true;
    postgres.enable = true;
    secrets.enable = true;

    kennel.services.tartan-vote.api = {
      customDomain = "api.tartan-vote.scottylabs.org";
      oidc.redirectPaths = [ "/oauth2/callback" ];
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
    backend.exec = "secretspec run --profile dev -- cargo run";
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
  '';

  env = {
    # Non-secret, machine-independent constants. Host-specific URLs are derived
    # from DEV_HOST in enterShell above.
    OIDC_ISSUER = "https://idp.scottylabs.org/realms/scottylabs";
    BETTER_AUTH_PROVIDER_ID = "cmu-sso";
    VITE_BETTER_AUTH_PROVIDER_ID = "cmu-sso";
    BETTER_AUTH_PORT = "3005";
    BIND_ADDR = "0.0.0.0:8080";

    # Actual secrets, resolved from OpenBao via secretspec.
    BETTER_AUTH_SECRET = config.secretspec.secrets.BETTER_AUTH_SECRET or "";
    OIDC_CLIENT_ID = config.secretspec.secrets.OIDC_CLIENT_ID or "";
    OIDC_CLIENT_SECRET = config.secretspec.secrets.OIDC_CLIENT_SECRET or "";
  };
}

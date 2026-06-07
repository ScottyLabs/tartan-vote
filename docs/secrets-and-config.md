# Secrets & Configuration

This document explains where every piece of runtime configuration comes from, how
secrets are managed with OpenBao + secretspec, and how to point the app at a
non-`localhost` host for cross-machine testing.

## TL;DR

- Enter the dev environment with `devenv shell` (or `direnv`), then run
  `devenv up` to start the API, auth service, and frontend together.

- The shell provides constants, derived URLs, the database connection, and real
  secrets pulled from OpenBao.

- You normally do not need a `.env` file for local development.

- One-time per machine (per year): authenticate to OpenBao so secretspec can read secrets:

  ```bash
  export BAO_ADDR=https://secrets2.scottylabs.org
  bao login -method=oidc
  ```

- For cross-machine testing, set your LAN IP in a git-ignored `.env.local`:

  ```bash
  echo 'DEV_HOST=192.168.1.20' > .env.local
  ```

## Where configuration comes from

Configuration is split by *what kind of value it is*, so the shared secret store
never carries any one developer's machine-specific values.

| Source | What it provides | Defined in |
| --- | --- | --- |
| **devenv constants** | Non-secret, machine-independent values: `OIDC_ISSUER`, `BETTER_AUTH_PROVIDER_ID`, `VITE_BETTER_AUTH_PROVIDER_ID`, `BETTER_AUTH_PORT`, `BIND_ADDR` | `devenv.nix` (`env`) |
| **Derived host URLs** | `APP_BASE_URL`, `FRONTEND_BASE_URL`, `BETTER_AUTH_URL`, `BETTER_AUTH_BASE_URL`, `VITE_API_BASE`, `VITE_BETTER_AUTH_BASE_URL`, `CORS_ALLOWED_ORIGINS` | `devenv.nix` (`enterShell`), computed from `DEV_HOST` |
| **OpenBao (secretspec)** | Real secrets: `OIDC_CLIENT_ID`, `OIDC_CLIENT_SECRET`, `BETTER_AUTH_SECRET` | `secret/secretspec/tartan-vote/dev/*` |
| **devenv Postgres** | `DATABASE_URL` (points at the managed Postgres socket/port) | `devenv.nix` (`enterShell`) |
| **secretspec prod/staging/preview** | Deployment URLs + secrets for Kennel | `secretspec.toml` profiles; Kennel resolves by environment |

The host URLs are all just `{scheme}://{host}:{port}` built from a single
`DEV_HOST`, so they don't need to be stored anywhere. Only the host varies
between developers.

## OpenBao & secretspec

Secrets live in [OpenBao](https://openbao.org/) at `https://secrets2.scottylabs.org`
and are accessed through [secretspec](https://secretspec.dev/).

### Project, provider, and profile

- **Project name:** `tartan-vote` (declared in `secretspec.toml` `[project] name`).
  This must match the OpenBao policy you have access to
  (`tartan-vote-dev`, `tartan-vote-prod`).
- **Provider:** `vault://secrets2.scottylabs.org/secret` (the ScottyLabs devenv
  module configures this; it works for both Vault and OpenBao).
- **Profile:** `dev` for local development; `prod`, `staging`, or `preview` for Kennel deployments.

The devenv integration is enabled in `devenv.yaml`, which makes
`config.secretspec.secrets.*` resolve from OpenBao at shell entry.

### Authenticating (one-time per machine)

secretspec reads your token from `$VAULT_TOKEN` or `~/.vault-token`. Get one via
OIDC:

```bash
export BAO_ADDR=https://secrets2.scottylabs.org
bao login -method=oidc
```

Confirm you have access to the project path:

```bash
bao token capabilities secret/data/secretspec/tartan-vote/dev/OIDC_CLIENT_ID
# should include "read"
```

If it prints `deny`, your OpenBao identity hasn't been granted the
`tartan-vote-dev` policy yet. This probably means you haven't done Governance
registration yet, but if you have, ask DevOps.

### Secrets stored in OpenBao

Only genuine secrets are stored (see `secretspec.toml`):

| Key | Description |
| --- | --- |
| `OIDC_CLIENT_ID` | OIDC client ID for the Keycloak client |
| `OIDC_CLIENT_SECRET` | OIDC client secret for the Keycloak client |
| `BETTER_AUTH_SECRET` | Better Auth signing secret (`openssl rand -base64 32`) |

### Common commands

```bash
# Verify all required secrets are present for the active profile
secretspec check

# Read a single secret
secretspec get OIDC_CLIENT_ID

# Set / rotate a secret (prompts for the value)
secretspec set BETTER_AUTH_SECRET

# Run a command with secrets injected (outside devenv shell)
secretspec run -- <command>
```

> Inside `devenv shell`, secrets are already exported into the environment
> (`devenv.nix` maps them from `config.secretspec.secrets`). The devenv
> processes (`devenv up`) wrap commands in `secretspec run --profile dev` so
> they work even when started outside a fully-loaded shell.

Your global default provider/profile is in `~/.config/secretspec/config.toml`:

```toml
[defaults]
provider = "vault://secrets2.scottylabs.org/secret"
profile  = "dev"
```

## DEV_HOST & cross-machine testing

By default `DEV_HOST=localhost` and all URLs resolve to `http://localhost:<port>`.

To run the dev stack on one machine and use it from another device's browser, the
URLs must point at the serving machine's LAN IP — a remote browser resolves
`localhost` to *itself*, which breaks the OIDC redirect, cookies, and CORS.

1. Create a git-ignored `.env.local` in the repo root:

   ```bash
   DEV_HOST=192.168.1.20
   ```

   `devenv`'s `enterShell` sources this and re-derives every URL from it.
   Re-enter the shell (or `direnv reload`) to apply.

1. Register the callback on the Keycloak/OIDC client:

   ```
   http://192.168.1.20:8080/auth/callback
   ```

   This step is inherent to OIDC and cannot be derived automatically. Without it
   the IdP rejects the login with `Invalid parameter: redirect_uri`.

Remove `.env.local` (or comment the line) to return to `localhost`.

## Kennel deployment & OIDC auto-provisioning

Kennel deploys three artifacts declared in `devenv.nix`:

| Kind | Name | Package | Custom domain |
| --- | --- | --- | --- |
| Service | `api` | `packages.api` | `api.tartan-vote.scottylabs.org` |
| Service | `auth` | `packages.auth` | `auth.tartan-vote.scottylabs.org` |
| Site | `frontend` | `packages.frontend` | `tartan-vote.scottylabs.org` |

The `api` service declares `oidc.redirectPaths = [ "/auth/callback" ]`. On every
deploy, Kennel reconciles Keycloak clients and writes OIDC credentials to OpenBao:

| OpenBao path | Written by |
| --- | --- |
| `secret/secretspec/tartan-vote/prod/OIDC_CLIENT_ID` | Kennel (prod client `tartan-vote`) |
| `secret/secretspec/tartan-vote/prod/OIDC_CLIENT_SECRET` | Kennel |
| `secret/secretspec/tartan-vote/staging/OIDC_CLIENT_*` | Kennel (client `tartan-vote-staging`) |
| `secret/secretspec/tartan-vote/preview/OIDC_CLIENT_*` | Kennel (same staging credentials) |

Redirect URIs Kennel registers include
`https://tartan-vote-api-main.scottylabs.net/auth/callback` and
`https://api.tartan-vote.scottylabs.org/auth/callback` (plus staging/PR variants).

`BETTER_AUTH_SECRET` for prod/staging must be seeded manually in OpenBao before
the first deploy:

```bash
bao kv put secret/secretspec/tartan-vote/prod/BETTER_AUTH_SECRET value="$(openssl rand -base64 32)"
```

After a successful main-branch deploy, verify OIDC secrets:

```bash
bao kv get secret/secretspec/tartan-vote/prod/OIDC_CLIENT_ID
bao kv get secret/secretspec/tartan-vote/prod/OIDC_CLIENT_SECRET
```

Local development does **not** auto-create Keycloak clients. Dev uses the shared
`idp.scottylabs.org` realm with secrets from the `dev` profile; register
`http://$DEV_HOST:8080/auth/callback` manually for cross-machine testing.

## Troubleshooting

- **`devenv` reports `Missing required secrets: ...`**
  You haven't authenticated to OpenBao, or the secret isn't set. Run
  `bao login -method=oidc`, then `secretspec check`. Set any missing values with
  `secretspec set <KEY>`.

- **`403 permission denied` / `Vault authentication failed`**
  Your token lacks the `tartan-vote-dev` policy (or expired). Re-run
  `bao login -method=oidc` and verify with `bao token capabilities ...`. If still
  denied, verify that you have completed Governance, and then message DevOps.

- **`OIDC_ISSUER, OIDC_CLIENT_ID, and OIDC_CLIENT_SECRET must be set`**
  The auth-service couldn't read these. Inside `devenv shell` they come from
  constants + OpenBao; ensure you're in the shell and authenticated.

- **`Invalid parameter: redirect_uri`**
  `{APP_BASE_URL}/auth/callback` isn't allowlisted on the IdP client. Either set
  `DEV_HOST` to a registered host or add the callback URL to the Keycloak client.

- **Frontend hits the wrong API host after changing `DEV_HOST`**
  Vite reads the `VITE_*` vars from the shell at startup. Restart the frontend
  dev server (and re-enter `devenv shell`) after editing `.env.local`.

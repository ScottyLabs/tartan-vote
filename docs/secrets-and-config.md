# Secrets & Configuration

This document explains where every piece of runtime configuration comes from, how
secrets are managed with OpenBao + secretspec, and how to point the app at a
non-`localhost` host for cross-machine testing.

## TL;DR

- Enter the dev environment with `devenv shell` (or `direnv`). It provides
  everything: constants, derived URLs, the database connection, and real
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
- **Profile:** `dev` for local development.

The devenv integration is enabled with `secretspec.enable = true;` in `devenv.nix`,
which makes `config.secretspec.secrets.*` resolve from OpenBao at shell entry.

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
> (`devenv.nix` maps them from `config.secretspec.secrets`), so you run the
> backend and auth-service directly — `cargo run`, `bun run dev` — without
> wrapping them in `secretspec run`. `secretspec run` is only needed to inject
> secrets for a process started **outside** the dev shell.

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

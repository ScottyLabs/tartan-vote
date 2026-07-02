# Secrets & Configuration

This document explains where every piece of runtime configuration comes from and
how secrets are managed with OpenBao + secretspec.

## TL;DR

- Enter the dev environment with `devenv shell` (or `direnv`), start the
  managed Postgres with `devenv up`, then build the frontend (`deno task build`)
  and run the backend (`cargo run`).

- The shell provides the database connection and real secrets pulled from
  OpenBao.

- You do not need a `.env` file for local development.

- One-time per machine (per year): authenticate to OpenBao so secretspec can read secrets:

  ```bash
  export BAO_ADDR=https://secrets2.scottylabs.org
  bao login -method=oidc
  ```

## Where configuration comes from

The app is same-origin — the backend serves the built frontend — so it needs
almost no configuration. The backend reads:

| Variable | Required | Source |
| --- | --- | --- |
| `DATABASE_URL` | yes | devenv's managed Postgres locally; Kennel in deployment |
| `PORT` / `BIND_ADDR` | no (defaults to `0.0.0.0:8080`) | set by the deployment platform |
| `SENTRY_DSN` | no | secretspec `prod` profile |
| `FRONTEND_DIST` | no (falls back to the Nix-bundled path, then `frontend/dist`) | override for the built frontend directory |

Real secrets (`OIDC_CLIENT_ID`, `OIDC_CLIENT_SECRET`, and the
governance-provisioned `KEYCLOAK_URL`, `KEYCLOAK_REALM`, `OAUTH_RELAY_URL`,
`PROJECT_GROUP`, `PROJECT_ADMIN_GROUP`) live in OpenBao at
`secret/secretspec/tartan-vote/dev/*` and are reserved for the planned OIDC
flow.

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

### Common commands

```bash
# Verify all required secrets are present for the active profile
secretspec check

# Read a single secret
secretspec get OIDC_CLIENT_ID

# Set / rotate a secret (prompts for the value)
secretspec set OIDC_CLIENT_SECRET

# Run a command with secrets injected (outside devenv shell)
secretspec run -- <command>
```

> Inside `devenv shell`, secrets are already exported into the environment
> (`devenv.nix` maps them from `config.secretspec.secrets`), so `cargo run`
> works directly. Outside a fully-loaded shell, wrap commands in
> `secretspec run --profile dev -- <command>`.

Your global default provider/profile is in `~/.config/secretspec/config.toml`:

```toml
[defaults]
provider = "vault://secrets2.scottylabs.org/secret"
profile  = "dev"
```

## Kennel deployment & OIDC auto-provisioning

Kennel deploys one artifact declared in `devenv.nix`:

| Kind | Name | Package | Custom domain |
| --- | --- | --- | --- |
| Service | `tartan-vote` | `packages.tartan-vote` | `tartan-vote.scottylabs.org` |

The `tartan-vote` service serves both the API and the built frontend. It declares
`oidc.redirectPaths = [ "/auth/callback" ]`. On every
deploy, Kennel reconciles Keycloak clients and writes OIDC credentials to OpenBao:

| OpenBao path | Written by |
| --- | --- |
| `secret/secretspec/tartan-vote/prod/OIDC_CLIENT_ID` | Kennel (prod client `tartan-vote`) |
| `secret/secretspec/tartan-vote/prod/OIDC_CLIENT_SECRET` | Kennel |
| `secret/secretspec/tartan-vote/staging/OIDC_CLIENT_*` | Kennel (client `tartan-vote-staging`) |
| `secret/secretspec/tartan-vote/preview/OIDC_CLIENT_*` | Kennel (same staging credentials) |

Redirect URIs Kennel registers include
`https://tartan-vote-tartan-vote-main.scottylabs.net/auth/callback` and
`https://tartan-vote.scottylabs.org/auth/callback` (plus staging/PR variants).

After a successful main-branch deploy, verify OIDC secrets:

```bash
bao kv get secret/secretspec/tartan-vote/prod/OIDC_CLIENT_ID
bao kv get secret/secretspec/tartan-vote/prod/OIDC_CLIENT_SECRET
```

Local development does **not** auto-create Keycloak clients. Dev uses the shared
`idp.scottylabs.org` realm with secrets from the `dev` profile; register
`http://<host>:8080/auth/callback` manually for any host you serve from.

## Troubleshooting

- **`devenv` reports `Missing required secrets: ...`**
  You haven't authenticated to OpenBao, or the secret isn't set. Run
  `bao login -method=oidc`, then `secretspec check`. Set any missing values with
  `secretspec set <KEY>`.

- **`403 permission denied` / `Vault authentication failed`**
  Your token lacks the `tartan-vote-dev` policy (or expired). Re-run
  `bao login -method=oidc` and verify with `bao token capabilities ...`. If still
  denied, verify that you have completed Governance, and then message DevOps.

- **`Invalid parameter: redirect_uri`**
  The `/auth/callback` URL for the host you're serving from isn't allowlisted on
  the IdP client. Add it to the Keycloak client.

# Extensive Guide to Running Tartan Vote

## Prerequisites

This project uses [devenv](https://devenv.sh/getting-started/) to provide Cargo, Deno, Node, PostgreSQL, and all other development dependencies. Follow the devenv installation instructions.

## Starting up

Now, we will get your own instance of Tartan Vote running!

### Setup

You will need [git](https://git-scm.com/install/).

Clone the repository from Codeberg:

```bash
git clone https://codeberg.org/ScottyLabs/tartan-vote.git
cd tartan-vote
```

Run `direnv allow` (or `devenv shell`) to enter the development environment. This starts PostgreSQL and exposes Cargo, Deno, Node, and other tooling.

### Secrets

Configuration is provided automatically inside `devenv shell` — you do not
need to create a `.env`. Secrets are pulled from OpenBao via secretspec, so
authenticate once:

```bash
export BAO_ADDR=https://secrets2.scottylabs.org
bao login -method=oidc
```

If `devenv shell` reports missing secrets or you get `403 permission denied`, see
[secrets-and-config.md](secrets-and-config.md), which documents the full secrets
model, the `DEV_HOST` override for cross-machine testing, and troubleshooting.

### Better Auth migrations

On first setup (or after auth schema changes), run migrations from the auth service:

```bash
cd auth-service
npm run migrate
```

### Run everything

From the repo root, start all three processes with devenv:

```bash
devenv up
```

This runs:

- **api** — Rust backend (`cargo run`)
- **auth** — Better Auth service (`node server.mjs`)
- **frontend** — Svelte dev server (`deno run dev --host`)

You can also start processes individually:

```bash
devenv processes up api
devenv processes up auth
devenv processes up frontend
```

### Cross-machine testing

To serve on one machine and browse from another, set your LAN IP in a git-ignored `.env.local`:

```bash
echo 'DEV_HOST=192.168.1.20' > .env.local
```

Re-enter the shell, then register `http://<your-ip>:8080/auth/callback` on the Keycloak client. See [secrets-and-config.md](secrets-and-config.md).

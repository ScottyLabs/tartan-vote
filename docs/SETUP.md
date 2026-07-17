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

Run `direnv allow` (or `devenv shell`) to enter the development environment. This exposes Cargo, Deno, Node, PostgreSQL, and other tooling.

### Secrets

Configuration is provided automatically inside `devenv shell` - you do not
need to create a `.env`. Secrets are pulled from OpenBao via secretspec, so
authenticate once:

```bash
export BAO_ADDR=https://secrets2.scottylabs.org
bao login -method=oidc
```

If `devenv shell` reports missing secrets or you get `403 permission denied`, see
[secrets-and-config.md](secrets-and-config.md), which documents the full secrets
model and troubleshooting.

### Run everything

From the repo root, inside the devenv shell:

```bash
# 1. Start the managed services (Postgres, OAuth relay)
devenv up

# 2. In another terminal: build the frontend into frontend/dist
cd frontend && deno task build && cd ..

# 3. Run the backend; it serves the API and the built frontend on :8080
cargo run
```

Then open http://localhost:8080.

When working on the frontend, run `deno task build:watch` in a separate
terminal instead of the one-off build; it rebuilds `frontend/dist` on save, and
a browser refresh picks up the changes.

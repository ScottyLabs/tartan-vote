# Slightly More Extensive Guide to Running Tartan Vote

## Prerequisites

Install Lix by following kennel's [local development guide](https://docs.kennel.scottylabs.org/guides/local-development/#installing-nix). The dev shell provides Cargo, Deno, Node, PostgreSQL, and all other development dependencies.

## Starting up

Now, we will get your own instance of Tartan Vote running!

### Setup

You will need [git](https://git-scm.com/install/).

Clone the repository from git.cmu.dev:

```bash
git clone https://git.cmu.dev/ScottyLabs/tartan-vote.git
cd tartan-vote
```

Run `nix develop` to enter the development environment. This exposes Cargo, Deno, Node, PostgreSQL, and other tooling.

### Secrets

Configuration is provided automatically inside `nix develop`, so there's no need to create a `.env`. Secrets are pulled from OpenBao via secretspec, so log in as described in kennel's [secrets guide](https://docs.kennel.scottylabs.org/guides/secrets/#local-development).

### Run everything

From the repo root, inside the dev shell:

```bash
pitchfork start --local
```

This starts Postgres, the OAuth relay, a frontend build that reruns on file change, and the backend on :8080. Then open http://localhost:8080.

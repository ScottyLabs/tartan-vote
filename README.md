# Tartan Vote

Tartan Vote is a CMU Undergraduate Senate-commissioned, ScottyLabs-developed voting app, to help the Senate and other student organizations manage attendance and host elections and motions. Currently, the app is still under development, but we _strongly_ hope to get it completed very soon! <!-- Add information about where to access the website here, when the MVP is done! -->

### Built With

- Svelte
- Rust
- PostgreSQL
- Better Auth

## Assumptions about the reader

Hello, reader! For the remainder of this README, and other documentation, we will assume that you are a developer or contributor, using WSL or a Unix development system, and have some familiarity with the command line. If you need any help, you are free to contact one of the codeowners found in CODEOWNERS, or join the [discord](https://go.scottylabs.org/discord).

## Photos

![login page](docs/photos/login.png)

![join page](docs/photos/joinPage.png)

![proxySetup page](docs/photos/proxySetup.png)

![waiting page page](docs/photos/waitingPage.png)

![host overview page](docs/photos/meetingOverview.png)

![voting page](docs/photos/votingPage.png)

![liveResults page](docs/photos/liveResults.png)

## Getting Started

### Prerequisites

- [devenv](https://devenv.sh/getting-started/) - Provides Bun, Cargo, and PostgreSQL via Nix

### Quick Setup

For detailed setup instructions, see [SETUP.md](docs/SETUP.md). Configuration and
secrets are documented in [secrets-and-config.md](docs/secrets-and-config.md).

Inside `devenv shell`, all configuration (constants, host URLs, `DATABASE_URL`)
is provided automatically; real secrets come from OpenBao. Authenticate once per
machine with `bao login -method=oidc` (`BAO_ADDR=https://secrets2.scottylabs.org`).

#### Starting auth

```bash
# Install auth service dependencies
cd auth-service
bun install

# Run Better Auth migrations (first run / after auth schema changes)
bun run migrate

# Start Better Auth service (in a separate terminal)
bun run dev
```

### Running the backend

Run the backend from the repo root. The workspace's default-members points at
the backend crate, so no `cd` needed.

```
cargo run
```

If you so desire, you can always go to the package directory, I guess...

#### Starting the frontend

```bash
cd frontend
bun install
bun run dev
```

### Contributing

Please check [CONTRIBUTING.md](docs/CONTRIBUTING.md) before you contribute to this project!

### Licenses

Voting App is distributed under the Apache 2.0 and MIT Licenses, found in the files `LICENSE-APACHE-2.0` and `LICENSE-MIT` respectively.

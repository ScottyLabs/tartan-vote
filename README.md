# Poodle

Poodle is a CMU Undergraduate Senate-commissioned, ScottyLabs-developed voting app, to help the Senate and other student organizations manage attendance and host elections and motions. Currently, the app is still under development, but we _strongly_ hope to get it completed very soon! <!-- Add information about where to access the website here, when the MVP is done! -->

### Built With

- Svelte
- Rust
- PostgreSQL
- Better Auth

## Assumptions about the reader

Hello, reader! For the remainder of this README, and other documentation, we will assume that you are a developer or contributor, using WSL or a Unix development system, and have some familiarity with the command line. If you need any help, you are free to contact one of the codeowners found in .github/CODEOWNERS, or join the [discord](https://go.scottylabs.org/discord).

## Getting Started

### Prerequisites

- [devenv](https://devenv.sh/getting-started/) - Provides Bun, Cargo, and PostgreSQL via Nix

### Quick Setup

For detailed setup instructions, see [SETUP.md](docs/SETUP.md).

#### Starting the backend

```bash
# Copy the .env.example
cp .env.example .env

# Install auth service dependencies
cd auth-service
bun install

# Run Better Auth migrations (first run / after auth schema changes)
bun x auth@latest migrate --config ./auth.mjs

# Start Better Auth service (in a separate terminal)
bun run dev

# Run the backend (postgres is managed by devenv)
cd backend/crates/voting-app
cargo run
```

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

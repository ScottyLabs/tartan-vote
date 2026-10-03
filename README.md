# Tartan Vote

Tartan Vote is a CMU Undergraduate Senate-commissioned, ScottyLabs-developed voting app, to help the Senate and other student organizations manage attendance and host elections and motions. Currently, the app is still under development, but we _strongly_ hope to get it completed very soon! <!-- Add information about where to access the website here, when the MVP is done! -->

### Built With

- SvelteKit
- Rust
- PostgreSQL

## Assumptions about the reader

Hello, reader! For the remainder of this README, and other documentation, we will assume that you are a developer or contributor, using WSL or a Unix development system, and have some familiarity with the command line. If you need any help, you are free to contact one of the codeowners found in CODEOWNERS, or join the [discord](https://go.scottylabs.org/discord).

## Getting Started

### Prerequisites

- You are added to the Tartan-Vote team on [governance](https://git.cmu.dev/ScottyLabs/governance)
- [devenv](https://devenv.sh/getting-started/) provides Cargo, Deno, Node, PostgreSQL, and other tooling via Nix

### Quick Setup

For detailed setup instructions, see [SETUP.md](docs/SETUP.md).

Authenticate once per machine with OpenBao so secretspec can read dev secrets:

```bash
nix run git+https://git.cmu.dev/ScottyLabs/kennel#login
```

Allow devenv (or enter the shell manually):

```bash
devenv allow
# or: devenv shell
```

Run the app (inside the devenv shell, from the repo root):

```bash
devenv up
# add --detach or -d to run it in the background
# devenv processes down to shut it down

# in a separate terminal if you didn't add -d
cd frontend && deno task build

cargo run
```

Then open http://localhost:8080.

### Deployment

Production runs on [Kennel](https://git.cmu.dev/ScottyLabs/kennel) via devenv and secretspec.

### Contributing

Please check [CONTRIBUTING.md](docs/CONTRIBUTING.md) before you contribute to this project!

### Licenses

Voting App is distributed under the Apache 2.0 and MIT Licenses, found in the files `LICENSE-APACHE-2.0` and `LICENSE-MIT` respectively.

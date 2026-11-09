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

- You are on the Tartan Vote team, which you can [join through governance](https://git.cmu.dev/ScottyLabs/governance#joining-a-team)
- Lix, installed by following kennel's [local development guide](https://docs.kennel.scottylabs.org/guides/local-development/#installing-nix). The dev shell provides Cargo, Deno, Node, PostgreSQL, and other tooling.

### Quick Setup

For detailed setup instructions, see [SETUP.md](docs/SETUP.md).

Log in to OpenBao as described in kennel's [secrets guide](https://docs.kennel.scottylabs.org/guides/secrets/#local-development) so secretspec can read dev secrets.

Enter the dev shell and start PostgreSQL, the OAuth relay, the frontend build in watch mode, and the backend:

```bash
nix develop
pitchfork start --local
```

Then open http://localhost:8080.

### Deployment

Production runs on [Kennel](https://docs.kennel.scottylabs.org/guides/deploying/), which builds the flake and loads secrets with secretspec.

### Contributing

Please check [CONTRIBUTING.md](docs/CONTRIBUTING.md) before you contribute to this project!

### Licenses

Voting App is distributed under the Apache 2.0 and MIT Licenses, found in the files `LICENSE-APACHE-2.0` and `LICENSE-MIT` respectively.

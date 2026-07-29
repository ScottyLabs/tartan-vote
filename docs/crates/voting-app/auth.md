# Auth

Authentication is **OIDC via Ricochet/Keycloak**: `/auth/login`, `/auth/callback`,
`/auth/logout`. OIDC secrets are provided by secretspec.

## Backend flow

1. The frontend links to `/auth/login`, which sits behind `OidcLoginLayer`;
   axum-oidc redirects the browser to Keycloak via the Ricochet relay. The OAuth
   `state` carries a CSRF token plus the app callback (`{APP_URL}/auth/callback`).
1. Keycloak authenticates the user; the relay forwards the code to
   `/auth/callback`, served by `axum_oidc::handle_oidc_redirect`.
1. The callback exchanges the code for tokens and stores them in a server-side
   session (Valkey, via `tower-sessions`).
1. `OidcAuthLayer` establishes the claims on each request; `sync_user_middleware`
   upserts a local `user` keyed on the OIDC subject and exposes it as `SyncedUser`.
1. The frontend reads `/auth/status`.

Logout (`GET /auth/logout`) flushes the local session and returns to the app
root. The Keycloak SSO session is left intact, so re-login does not re-prompt for
credentials.

### Sessions

Server-side sessions are backed by Valkey (`tower-sessions-redis-store` over
`fred`), connected via `VALKEY_URL`. Only the session token set
lives in Valkey user identity stays in Postgres, re-derived from the token
subject per request via `sync_user_middleware`.

### Files

- `src/core/auth/oidc.rs`: `GroupClaims`, the `SessionWrapper` bridge from
  `tower-sessions` to axum-oidc's session contract, the relay `state` generator,
  and the discovered `OidcClient` builder.
- `src/core/auth/middleware.rs`: `SyncedUser` and its extractors, plus
  `sync_user_middleware`.
- `src/domain/auth/handlers.rs`: `GET /auth/status`, the `/auth/login` and
  `/auth/logout` handlers, and the demo page.
- `src/server.rs`: mounts the session layer, `OidcAuthLayer`,
  `sync_user_middleware`, and CORS.

## Config

Secrets are loaded through the [secretspec](https://secretspec.dev/) Rust SDK
(`declare_secrets!` against the repo-root `secretspec.toml`), using the read-only
`env` provider.

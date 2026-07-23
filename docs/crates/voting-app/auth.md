# Auth

Two paths exist:

- **OIDC via Ricochet/Keycloak** (real SSO): `/auth/login`, `/auth/callback`,
  `/auth/logout`. OIDC secrets are provided by secretspec.
- **Dev bypass**: a cookie/header shortcut for local development and tests that
  creates/loads a local `user` and exposes it as `SyncedUser`. This bypass is scheduled to be removed in the future

## Backend flow

1. The frontend links to `/auth/login`, which sits behind `OidcLoginLayer`;
   axum-oidc redirects the browser to Keycloak via the Ricochet relay. The OAuth
   `state` carries a CSRF token plus the app callback (`{APP_URL}/auth/callback`).
1. Keycloak authenticates the user; the relay forwards the code to
   `/auth/callback`, served by `axum_oidc::handle_oidc_redirect`.
1. The callback exchanges the code for tokens and stores them in a server-side
   session (Postgres, via `tower-sessions`).
1. `OidcAuthLayer` establishes the claims on each request; `sync_user_middleware`
   upserts a local `user` keyed on the OIDC subject and exposes it as `SyncedUser`.
1. The frontend reads `/auth/status`.

Logout (`GET /auth/logout`) flushes the local session and returns to the app
root. The Keycloak SSO session is left intact, so re-login does not re-prompt for
credentials.

### Sessions

Server-side sessions are backed by the existing Postgres connection
(`tower-sessions-sqlx-store`), so they survive restarts and stay consistent
across instances. The session cookie holds only an opaque id; the OIDC token set
lives in the session store under the `axum-oidc` key. There is no foreign key
from the session table to `user`; identity is re-derived from the token subject
per request. Contrast with the domain `session` / `user_session` tables, which
are voting sessions, unrelated to auth.

### Files

- `src/core/auth/oidc.rs`: `GroupClaims`, the `SessionWrapper` bridge from
  `tower-sessions` to axum-oidc's session contract, the relay `state` generator,
  and the discovered `OidcClient` builder.
- `src/core/auth/middleware.rs`: `SyncedUser` and its extractors, plus
  `sync_user_middleware`.
- `src/domain/auth/bypass.rs`: `POST /auth/bypass/login` (creates/loads a user
  with `oidc_subject = "bypass:<andrew_id>"`), `GET /auth/bypass/status`,
  `POST /auth/bypass/logout`, and `bypass_auth_middleware`.
- `src/domain/auth/handlers.rs`: `GET /auth/status`, the `/auth/login` and
  `/auth/logout` handlers, and the demo page.
- `src/server.rs`: mounts the session layer, `OidcAuthLayer`,
  `sync_user_middleware`, the bypass middleware, and CORS.

## Config

Secrets are loaded through the [secretspec](https://secretspec.dev/) Rust SDK
(`declare_secrets!` against the repo-root `secretspec.toml`), using the read-only
`env` provider.

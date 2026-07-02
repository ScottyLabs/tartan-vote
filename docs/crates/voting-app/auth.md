# Auth

Better Auth has been removed. The backend no longer talks to an external auth
service; user identity is resolved entirely in-process.

Two paths exist today:

- **Dev bypass** (working): a cookie/header-based shortcut for local development
  and tests that creates/loads a local `user` and exposes it as `SyncedUser`.
- **OIDC via Ricochet/Keycloak** (planned, stubbed): the `/auth/login` and
  `/auth/callback` endpoints are placeholders. Real SSO through the Ricochet OAuth
  relay is not yet implemented.

## Backend flow

### `src/core/auth/middleware.rs`

- Defines `SyncedUser(Arc<user::Model>)` and its `FromRequestParts` /
  `OptionalFromRequestParts` extractors. Handlers require auth by extracting
  `SyncedUser`, or read optional auth via `Option<SyncedUser>`.
- No session is fetched here anymore; `SyncedUser` is populated by the bypass
  middleware (and, eventually, by the OIDC callback).

### `src/domain/auth/bypass.rs`

- `POST /auth/bypass/login`: creates/loads a user (`oidc_subject = "bypass:<andrew_id>"`)
  and sets a `bypass_user_id` cookie.
- `GET /auth/bypass/status`, `POST /auth/bypass/logout`: report / clear the bypass
  session.
- `bypass_auth_middleware`: resolves `SyncedUser` from the `bypass_user_id`
  cookie or the `x-bypass-user-id` header.

### `src/domain/auth/handlers.rs`

- `GET /auth/status`: returns auth state derived from optional `SyncedUser`.
- `GET /auth/login`, `/auth/logout`: redirect to the frontend (stubs).
- `GET /auth/callback`: stub that redirects to the frontend. **TODO:** exchange
  the authorization code for tokens via the Ricochet relay / Keycloak, establish
  a session, then redirect.

### `src/server.rs`

- Mounts the bypass-auth middleware globally. There is no CORS layer: the
  backend serves the built frontend, so all requests are same-origin.
- The `tartan-vote` service declares `oidc.redirectPaths = [ "/auth/callback" ]` in
  `devenv.nix`; the endpoint is reserved for the future OIDC flow.

## Env vars

Auth-related configuration is provided automatically inside `devenv shell` (see
[secrets-and-config.md](../../secrets-and-config.md)):

- `OIDC_CLIENT_ID`, `OIDC_CLIENT_SECRET` — **OpenBao secrets** (governance
  `oidc_client` feature).
- `KEYCLOAK_URL`, `KEYCLOAK_REALM`, `OAUTH_RELAY_URL`, `PROJECT_GROUP`,
  `PROJECT_ADMIN_GROUP` — provisioned by governance for the OIDC flow.

None of these are read by the backend yet; they are reserved for the planned
OIDC implementation. The stubs redirect to `/` (same-origin), so no base-URL
configuration is needed.

The local OAuth relay is enabled with `scottylabs.ricochet.enable` in
`devenv.nix`; it exports `APP_URL` so the relay can return to the backend
`/auth/callback` once the OIDC flow is implemented.

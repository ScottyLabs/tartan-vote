use axum::{
    Json,
    response::{Html, IntoResponse, Redirect},
};
use serde::Serialize;
use tower_sessions::Session;
use utoipa::ToSchema;

use crate::core::auth::middleware::SyncedUser;

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthStatusResponse {
    pub logged_in: bool,
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub user_andrew_id: Option<String>,
}

pub async fn login() -> impl IntoResponse {
    Redirect::to("/")
}

/// OIDC callback fallback used only when OIDC is not configured. When it is, the
/// route is served by `axum_oidc::handle_oidc_redirect` instead (see `server.rs`).
pub async fn callback() -> impl IntoResponse {
    Redirect::to("/")
}

/// Clears the server-side session and returns to the app root. The Keycloak SSO
/// session is left intact, so signing back in does not re-prompt for credentials.
pub async fn logout(session: Session) -> impl IntoResponse {
    let _ = session.flush().await;
    Redirect::to("/")
}

#[utoipa::path(
    get,
    path = "/auth/status",
    tag = "auth",
    responses(
        (status = 200, description = "Current authentication status", body = AuthStatusResponse)
    )
)]
pub async fn auth_status(user: Option<SyncedUser>) -> impl IntoResponse {
    let payload = AuthStatusResponse {
        logged_in: user.is_some(),
        user_id: user.clone().map(|u| u.0.id),
        user_name: user.clone().map(|u| u.0.name.clone()),
        user_andrew_id: user.map(|u| u.0.andrew_id.clone()),
    };
    Json(payload)
}

const BYPASS_FORM_HTML: &str = "\
        <h2>Auth Bypass</h2>
        <form id=\"bypass-form\">
            <label>Name <input id=\"bp-name\" value=\"Demo User\" /></label>
            <label>Andrew ID <input id=\"bp-andrew\" value=\"demo\" /></label>
            <button type=\"submit\">Bypass Login</button>
            <button type=\"button\" id=\"bp-status\">Refresh Status</button>
            <button type=\"button\" id=\"bp-logout\">Bypass Logout</button>
        </form>
        <pre id=\"bp-out\"></pre>";

const BYPASS_JS: &str = "\
        const out = document.getElementById('bp-out');
        async function show(req) {
            try {
                const res = await req;
                const text = await res.text();
                out.textContent = res.status + ' ' + text;
            } catch (err) {
                out.textContent = String(err);
            }
        }
        document.getElementById('bypass-form').addEventListener('submit', (e) => {
            e.preventDefault();
            show(fetch('/auth/bypass/login', {
                method: 'POST',
                credentials: 'include',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    name: document.getElementById('bp-name').value,
                    andrew_id: document.getElementById('bp-andrew').value,
                }),
            }));
        });
        document.getElementById('bp-status').addEventListener('click', () =>
            show(fetch('/auth/bypass/status', { credentials: 'include' })));
        document.getElementById('bp-logout').addEventListener('click', () =>
            show(fetch('/auth/bypass/logout', { method: 'POST', credentials: 'include' })));";

pub async fn demo_home() -> impl IntoResponse {
    let bypass_section = BYPASS_FORM_HTML;
    let bypass_script = format!("<script>{BYPASS_JS}</script>");

    let html = format!(
        "<!doctype html>
<html>
    <head>
        <meta charset=\"utf-8\" />
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
        <title>Voting App Backend Demo</title>
    </head>
    <body>
        <ul>
            <li><a href=\"/auth/login\">Login</a></li>
            <li><a href=\"/auth/logout\">Logout</a></li>
            <li><a href=\"/auth/status\">Auth Status (JSON)</a></li>
            <li><a href=\"/health\">Health</a></li>
        </ul>
        {bypass_section}
        {bypass_script}
    </body>
</html>"
    );

    Html(html)
}

pub async fn demo_not_found() -> impl IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        Html("<!doctype html><html><body><h1>Not Found</h1></body></html>"),
    )
}

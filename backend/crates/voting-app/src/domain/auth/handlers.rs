use axum::{
    Json,
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect},
};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims, OidcRpInitiatedLogout};
use http::Uri;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::AppState;
use crate::core::auth::middleware::SyncedUser;

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthStatusResponse {
    pub logged_in: bool,
    pub user_id: Option<i32>,
}

pub async fn login(
    State(state): State<AppState>,
    Query(params): Query<LoginQuery>,
    session: Session,
) -> impl IntoResponse {
    let callback = format!(
        "{}/auth/callback",
        state.config.app_base_url.trim_end_matches('/')
    );

    if let Some(redirect_uri) = params.redirect_uri {
        session
            .insert("post_login_redirect", redirect_uri)
            .await
            .ok();
    }

    Redirect::to(&callback)
}

pub async fn callback(
    _claims: OidcClaims<EmptyAdditionalClaims>,
    user: SyncedUser,
    State(state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    let _user_id = user.0.id;

    let redirect_to = session
        .remove::<String>("post_login_redirect")
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| state.config.frontend_base_url.clone());

    Redirect::to(&redirect_to)
}

pub async fn logout(
    logout: OidcRpInitiatedLogout,
    State(state): State<AppState>,
) -> impl IntoResponse {
    logout
        .with_post_logout_redirect(
            Uri::from_maybe_shared(state.config.app_base_url.clone()).expect("valid APP_BASE_URL"),
        )
        .into_response()
}

pub async fn auth_status(
    claims: Option<OidcClaims<EmptyAdditionalClaims>>,
    user: Option<SyncedUser>,
) -> impl IntoResponse {
    let payload = AuthStatusResponse {
        logged_in: claims.is_some(),
        user_id: user.map(|u| u.0.id),
    };
    Json(payload)
}

pub async fn demo_home(State(state): State<AppState>) -> impl IntoResponse {
    let base = state.config.app_base_url.trim_end_matches('/');
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
            <li><a href=\"{base}/auth/login\">Login</a></li>
            <li><a href=\"{base}/auth/logout\">Logout</a></li>
            <li><a href=\"{base}/auth/status\">Auth Status (JSON)</a></li>
            <li><a href=\"{base}/health\">Health</a></li>
        </ul>
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

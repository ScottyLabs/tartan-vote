use axum::{
    Json,
    extract::{RawQuery, State},
    response::{Html, IntoResponse, Redirect},
};
use serde::Serialize;

use crate::AppState;
use crate::core::auth::middleware::SyncedUser;

#[derive(Debug, Serialize)]
pub struct AuthStatusResponse {
    pub logged_in: bool,
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub user_andrew_id: Option<String>,
}

pub async fn login(State(state): State<AppState>) -> impl IntoResponse {
    Redirect::to(&state.config.frontend_base_url)
}

pub async fn callback(
    State(state): State<AppState>,
    RawQuery(raw_query): RawQuery,
) -> impl IntoResponse {
    if let Some(query) = raw_query {
        let target = format!(
            "{}/oauth2/callback/{}?{}",
            state.config.better_auth_base_url.trim_end_matches('/'),
            state.config.better_auth_provider_id,
            query
        );
        return Redirect::to(&target);
    }

    Redirect::to(&state.config.frontend_base_url)
}

pub async fn logout(State(state): State<AppState>) -> impl IntoResponse {
    Redirect::to(&state.config.frontend_base_url)
}

pub async fn auth_status(user: Option<SyncedUser>) -> impl IntoResponse {
    let payload = AuthStatusResponse {
        logged_in: user.is_some(),
        user_id: user.clone().map(|u| u.0.id),
        user_name: user.clone().map(|u| u.0.name.clone()),
        user_andrew_id: user.map(|u| u.0.andrew_id.clone()),
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

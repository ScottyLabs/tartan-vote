use axum::{
    Json,
    extract::{RawQuery, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse, Redirect},
};
use entity::{prelude::User, user};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::AppState;
use crate::core::auth::middleware::{DEV_SESSION_COOKIE, SyncedUser};

const DEV_OIDC_SUBJECT: &str = "dev-bypass-local";
const DEV_USER_NAME: &str = "Sample Voter";
const DEV_ANDREW_ID: &str = "samplevoter";

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

pub async fn dev_signin(State(state): State<AppState>) -> impl IntoResponse {
    if !state.config.dev_auth_bypass {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "message": "Dev auth bypass is disabled" })),
        )
            .into_response();
    }

    let user = match User::find()
        .filter(user::Column::OidcSubject.eq(DEV_OIDC_SUBJECT))
        .one(&state.db)
        .await
    {
        Ok(Some(existing)) => existing,
        Ok(None) => {
            let new_user = user::ActiveModel {
                name: Set(DEV_USER_NAME.to_string()),
                andrew_id: Set(DEV_ANDREW_ID.to_string()),
                oidc_subject: Set(DEV_OIDC_SUBJECT.to_string()),
                ..Default::default()
            };

            match new_user.insert(&state.db).await {
                Ok(created) => created,
                Err(err) => {
                    tracing::error!("failed to create dev user: {:?}", err);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "message": "Failed to create dev user" })),
                    )
                        .into_response();
                }
            }
        }
        Err(err) => {
            tracing::error!("failed to look up dev user: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "message": "Failed to look up dev user" })),
            )
                .into_response();
        }
    };

    let cookie = format!("{DEV_SESSION_COOKIE}={}; Path=/; HttpOnly; SameSite=Lax", user.id);

    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(AuthStatusResponse {
            logged_in: true,
            user_id: Some(user.id),
            user_name: Some(user.name.clone()),
            user_andrew_id: Some(user.andrew_id.clone()),
        }),
    )
        .into_response()
}

pub async fn dev_signout(State(state): State<AppState>) -> impl IntoResponse {
    if !state.config.dev_auth_bypass {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "message": "Dev auth bypass is disabled" })),
        )
            .into_response();
    }

    let cookie = format!(
        "{DEV_SESSION_COOKIE}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"
    );

    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(serde_json::json!({ "logged_in": false })),
    )
        .into_response()
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

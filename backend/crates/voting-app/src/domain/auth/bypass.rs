use std::sync::Arc;

use axum::{
    Json, body,
    extract::State,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use entity::{prelude::User, user};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use crate::core::error::AppError;

const BYPASS_COOKIE: &str = "bypass_user_id";
const BYPASS_HEADER: &str = "x-bypass-user-id";

#[derive(Debug, Deserialize, ToSchema)]
pub struct BypassLoginRequest {
    pub name: String,
    pub andrew_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthBypassStatusResponse {
    pub logged_in: bool,
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub user_andrew_id: Option<String>,
}

fn status_payload(user: Option<&SyncedUser>) -> AuthBypassStatusResponse {
    AuthBypassStatusResponse {
        logged_in: user.is_some(),
        user_id: user.map(|u| u.0.id),
        user_name: user.map(|u| u.0.name.clone()),
        user_andrew_id: user.map(|u| u.0.andrew_id.clone()),
    }
}

#[utoipa::path(
    post,
    path = "/auth/bypass/login",
    tag = "auth",
    request_body = BypassLoginRequest,
    responses(
        (status = 200, description = "Logged in as the (bypassed) user", body = AuthBypassStatusResponse)
    )
)]
pub async fn bypass_login(
    State(state): State<AppState>,
    Json(req): Json<BypassLoginRequest>,
) -> Result<Response, AppError> {
    let oidc_subject = format!("bypass:{}", req.andrew_id);

    let existing = User::find()
        .filter(user::Column::OidcSubject.eq(&oidc_subject))
        .one(&state.db)
        .await?;

    let user = match existing {
        Some(user) => user,
        None => {
            let new_user = user::ActiveModel {
                name: Set(req.name),
                andrew_id: Set(req.andrew_id),
                oidc_subject: Set(oidc_subject),
                ..Default::default()
            };
            new_user.insert(&state.db).await?
        }
    };

    let synced = SyncedUser(Arc::new(user));
    let payload = status_payload(Some(&synced));

    let cookie = format!(
        "{BYPASS_COOKIE}={}; Path=/; Max-Age=604800; HttpOnly; SameSite=Lax",
        synced.0.id
    );

    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(payload),
    )
        .into_response())
}

#[utoipa::path(
    get,
    path = "/auth/bypass/status",
    tag = "auth",
    responses(
        (status = 200, description = "Current (bypassed) authentication status", body = AuthBypassStatusResponse)
    )
)]
pub async fn bypass_status(user: Option<SyncedUser>) -> Response {
    Json(status_payload(user.as_ref())).into_response()
}

#[utoipa::path(
    post,
    path = "/auth/bypass/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Cleared the bypass session", body = AuthBypassStatusResponse),
    )
)]
pub async fn bypass_logout() -> Response {
    let cookie = format!("{BYPASS_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax");

    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(status_payload(None)),
    )
        .into_response()
}

pub async fn bypass_auth_middleware(
    State(state): State<AppState>,
    mut request: http::Request<body::Body>,
    next: Next,
) -> Response {
    if request.extensions().get::<SyncedUser>().is_some() {
        return next.run(request).await;
    }

    if let Some(id) = bypass_user_id(&request) {
        match User::find_by_id(id).one(&state.db).await {
            Ok(Some(user)) => {
                request.extensions_mut().insert(SyncedUser(Arc::new(user)));
            }
            Ok(None) => {}
            Err(err) => tracing::error!("failed to load bypass user {id}: {:?}", err),
        }
    }

    next.run(request).await
}

fn bypass_user_id(request: &http::Request<body::Body>) -> Option<i32> {
    if let Some(id) = request
        .headers()
        .get(BYPASS_HEADER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.trim().parse::<i32>().ok())
    {
        return Some(id);
    }

    let cookie_header = request
        .headers()
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())?;

    cookie_header
        .split(';')
        .filter_map(|pair| pair.split_once('='))
        .find_map(|(name, value)| {
            (name.trim() == BYPASS_COOKIE)
                .then(|| value.trim().parse::<i32>().ok())
                .flatten()
        })
}

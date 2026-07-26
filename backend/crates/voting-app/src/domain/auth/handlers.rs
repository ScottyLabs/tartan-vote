use axum::{
    Json,
    response::{IntoResponse, Redirect},
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

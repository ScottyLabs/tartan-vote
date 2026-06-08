use axum::{
    body,
    extract::{FromRequestParts, OptionalFromRequestParts, State},
    http::header,
    middleware::Next,
    response::Response,
};
use entity::{prelude::User, user};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

use crate::AppState;

#[derive(Clone)]
pub struct SyncedUser(pub Arc<user::Model>);

#[derive(Clone, Deserialize)]
pub struct BetterAuthUser {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct BetterAuthSession {
    pub user: BetterAuthUser,
}

#[derive(Deserialize)]
struct BetterAuthGetSessionEnvelope {
    user: Option<BetterAuthUser>,
    data: Option<BetterAuthGetSessionEnvelopeData>,
}

#[derive(Deserialize)]
struct BetterAuthGetSessionEnvelopeData {
    user: Option<BetterAuthUser>,
}

impl<S> FromRequestParts<S> for SyncedUser
where
    S: Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<SyncedUser>()
            .cloned()
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)
    }
}

impl<S> OptionalFromRequestParts<S> for SyncedUser
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(parts.extensions.get::<SyncedUser>().cloned())
    }
}

pub const DEV_SESSION_COOKIE: &str = "dev_user_id";

pub async fn sync_user_middleware(
    State(state): State<AppState>,
    mut request: http::Request<body::Body>,
    next: Next,
) -> Response {
    if request.extensions().get::<SyncedUser>().is_some() {
        return next.run(request).await;
    }

    let cookie_header = request
        .headers()
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok());

    if state.config.dev_auth_bypass {
        if let Some(cookie_header) = cookie_header {
            if let Some(user) = load_dev_session_user(&state, cookie_header).await {
                request.extensions_mut().insert(SyncedUser(Arc::new(user)));
                return next.run(request).await;
            }
        }
    }

    let Some(cookie_header) = cookie_header else {
        return next.run(request).await;
    };

    let Some(session) = fetch_better_auth_session(&state, cookie_header).await else {
        return next.run(request).await;
    };

    let subject = session.user.id;
    let name = session
        .user
        .name
        .or_else(|| session.user.email.clone())
        .unwrap_or_else(|| "Unknown User".to_string());

    let andrewid = session
        .user
        .email
        .as_deref()
        .and_then(|email| email.split('@').next())
        .unwrap_or("unknown")
        .to_string();

    let user = User::find()
        .filter(user::Column::OidcSubject.eq(&subject))
        .one(&state.db)
        .await
        .ok()
        .flatten();

    match user {
        Some(user) => {
            request.extensions_mut().insert(SyncedUser(Arc::new(user)));
        }
        None => {
            let new_user = user::ActiveModel {
                name: Set(name),
                andrew_id: Set(andrewid),
                oidc_subject: Set(subject.clone()),
                ..Default::default()
            };

            match new_user.insert(&state.db).await {
                Ok(created) => {
                    request
                        .extensions_mut()
                        .insert(SyncedUser(Arc::new(created)));
                }
                Err(err) => {
                    tracing::error!("failed to create user from oidc claims: {:?}", err);
                }
            }
        }
    }

    next.run(request).await
}

async fn load_dev_session_user(state: &AppState, cookie_header: &str) -> Option<user::Model> {
    let user_id = cookie_header
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix(&format!("{DEV_SESSION_COOKIE}=")))?
        .parse::<i32>()
        .ok()?;

    User::find_by_id(user_id)
        .one(&state.db)
        .await
        .ok()
        .flatten()
}

async fn fetch_better_auth_session(
    state: &AppState,
    cookie_header: &str,
) -> Option<BetterAuthSession> {
    let endpoint = format!(
        "{}/get-session",
        state.config.better_auth_base_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let response = client
        .get(endpoint)
        .header(reqwest::header::COOKIE, cookie_header)
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let payload = response.json::<BetterAuthGetSessionEnvelope>().await.ok()?;

    let user = payload.user.or_else(|| payload.data.and_then(|d| d.user))?;

    Some(BetterAuthSession { user })
}

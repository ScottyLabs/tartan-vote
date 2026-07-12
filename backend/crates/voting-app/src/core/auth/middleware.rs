use axum::{
    body,
    extract::{FromRequestParts, OptionalFromRequestParts, State},
    middleware::Next,
    response::Response,
};
use axum_oidc::OidcClaims;
use entity::{prelude::User, user};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use std::convert::Infallible;
use std::sync::Arc;

use crate::AppState;
use crate::core::auth::oidc::GroupClaims;

#[derive(Clone)]
pub struct SyncedUser(pub Arc<user::Model>);

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

pub async fn sync_user_middleware(
    State(state): State<AppState>,
    request: http::Request<body::Body>,
    next: Next,
) -> Response {
    if request.extensions().get::<SyncedUser>().is_some() {
        return next.run(request).await;
    }

    let (mut parts, request_body) = request.into_parts();
    let claims =
        <OidcClaims<GroupClaims> as OptionalFromRequestParts<AppState>>::from_request_parts(
            &mut parts, &state,
        )
        .await
        .ok()
        .flatten();
    let mut request = http::Request::from_parts(parts, request_body);

    if let Some(claims) = claims
        && let Some(user) = upsert_user_from_claims(&state, &claims).await
    {
        request.extensions_mut().insert(SyncedUser(Arc::new(user)));
    }

    next.run(request).await
}

/// Finds the local user matching the token's subject, creating one on first login.
async fn upsert_user_from_claims(
    state: &AppState,
    claims: &OidcClaims<GroupClaims>,
) -> Option<user::Model> {
    let subject = claims.subject().to_string();

    let email = claims.email().map(|email| email.as_str().to_owned());

    let name = claims
        .name()
        .and_then(|name| name.get(None))
        .map(|name| name.as_str().to_owned())
        .or_else(|| claims.preferred_username().map(|u| u.as_str().to_owned()))
        .or_else(|| email.clone())
        .unwrap_or_else(|| "Unknown User".to_string());

    let andrew_id = email
        .as_deref()
        .and_then(|email| email.split('@').next())
        .or_else(|| claims.preferred_username().map(|u| u.as_str()))
        .unwrap_or("unknown")
        .to_string();

    match User::find()
        .filter(user::Column::OidcSubject.eq(&subject))
        .one(&state.db)
        .await
    {
        Ok(Some(user)) => Some(user),
        Ok(None) => {
            let new_user = user::ActiveModel {
                name: Set(name),
                andrew_id: Set(andrew_id),
                oidc_subject: Set(subject),
                ..Default::default()
            };
            match new_user.insert(&state.db).await {
                Ok(created) => Some(created),
                Err(err) => {
                    tracing::error!("failed to create user from oidc claims: {:?}", err);
                    None
                }
            }
        }
        Err(err) => {
            tracing::error!("failed to look up user by oidc subject: {:?}", err);
            None
        }
    }
}

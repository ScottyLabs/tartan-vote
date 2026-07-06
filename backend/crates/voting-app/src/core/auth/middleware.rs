use axum::extract::{FromRequestParts, OptionalFromRequestParts};
use entity::user;
use std::convert::Infallible;
use std::sync::Arc;

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

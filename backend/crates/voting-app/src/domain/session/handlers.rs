use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use entity::enums::{JoinLeft, SessionStatus};
use entity::{session, user_session};
use random_string::generate;
use sea_orm::ActiveValue::Set;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct CreateSessionResponse {
    pub session_code: String,
}

pub async fn create_session(user: SyncedUser, State(state): State<AppState>) -> impl IntoResponse {
    let store = &state.store;

    let charset = "ABCDEFGHIJKLMOPQRSTUVWXYZ0123456789";
    let session_code = generate(6, charset);

    // logic should be implemented to verify that the join code is actually unique
    // but as of right now this is low priority because 36^6 > 2 billion
    // TODO: database logic to force uniqueness
    let session = session::ActiveModel {
        join_code: Set(session_code),
        status: Set(SessionStatus::Open),
        created_by_user_id: Set(user.0.id),
        ..Default::default()
    };
    match store.sessions().create(session).await {
        Ok(session) => (
            StatusCode::CREATED,
            Json(CreateSessionResponse {
                session_code: session.join_code,
            }),
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn status_session(
    _user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> impl IntoResponse {
    let store = &state.store;

    match store.sessions().find_by_join_code(session_code).await {
        Ok(Some(session)) => (
            StatusCode::OK,
            Json(json!({ "session_ended": session.status == SessionStatus::Closed })),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Session not found"})),
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn join_session(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> impl IntoResponse {
    let store = &state.store;

    match store.sessions().find_by_join_code(session_code).await {
        Ok(Some(session)) => {
            let new_user_session = user_session::ActiveModel {
                user_id: Set(user.0.id),
                session_id: Set(session.id),
                join_left: Set(JoinLeft::Joined),
                ..Default::default()
            };

            match store.user_sessions().create(new_user_session).await {
                Ok(_) => (StatusCode::OK).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            }
        }
        // is this correct? idk when the store will return None vs Err
        Ok(None) => (StatusCode::NOT_ACCEPTABLE).into_response(),
        Err(_) => (StatusCode::NOT_ACCEPTABLE).into_response(),
    }
}

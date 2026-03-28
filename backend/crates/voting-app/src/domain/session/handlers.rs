use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use entity::enums::{JoinLeft, SessionStatus};
use entity::{session, user_session};
use random_string::generate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct CreateSessionResponse {
    pub session_code: String,
}

#[derive(Serialize)]
pub struct EndSessionResponse {
    pub session_code: String,
    pub status: SessionStatus,
}

#[derive(Serialize)]
pub struct HasActiveEventResponse {
    pub session_code: String,
    pub has_active_event: bool,
    pub event_id: Option<i32>,
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
            if session.status != SessionStatus::Open {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Session is not open"})),
                )
                    .into_response();
            }

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
        Ok(None) => (StatusCode::NOT_FOUND).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn end_session(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> impl IntoResponse {
    let store = &state.store;

    let session = match store
        .sessions()
        .find_by_join_code(session_code.clone())
        .await
    {
        Ok(Some(session)) => session,
        Ok(None) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if session.created_by_user_id != user.0.id {
        return (StatusCode::FORBIDDEN).into_response();
    }

    if session.status == SessionStatus::Closed {
        return (
            StatusCode::OK,
            Json(EndSessionResponse {
                session_code,
                status: SessionStatus::Closed,
            }),
        )
            .into_response();
    }

    let mut session_to_update = session.into_active_model();
    session_to_update.status = Set(SessionStatus::Closed);

    match store.sessions().update(session_to_update).await {
        Ok(updated) => (
            StatusCode::OK,
            Json(EndSessionResponse {
                session_code: updated.join_code,
                status: updated.status,
            }),
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn has_active_event(
    _user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> impl IntoResponse {
    let store = &state.store;

    let session = match store
        .sessions()
        .find_by_join_code(session_code.clone())
        .await
    {
        Ok(Some(session)) => session,
        Ok(None) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    match store.events().find_active_by_session_id(session.id).await {
        Ok(Some(event)) => (
            StatusCode::OK,
            Json(HasActiveEventResponse {
                session_code,
                has_active_event: true,
                event_id: Some(event.id),
            }),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::OK,
            Json(HasActiveEventResponse {
                session_code,
                has_active_event: false,
                event_id: None,
            }),
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

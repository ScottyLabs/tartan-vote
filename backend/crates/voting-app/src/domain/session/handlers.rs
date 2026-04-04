use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use entity::enums::{JoinLeft, SessionStatus};
use entity::{session, user_session};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::iter::repeat_with;

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

#[derive(Deserialize)]
pub struct SetSessionProxyRequest {
    pub is_senator: bool,
    pub proxy_for: Option<String>,
}

#[derive(Serialize)]
pub struct SetSessionProxyResponse {
    pub vote_instance_count: usize,
    pub is_senator: bool,
    pub has_proxy: bool,
}

pub async fn create_session(user: SyncedUser, State(state): State<AppState>) -> impl IntoResponse {
    let store = &state.store;

    let session_code = repeat_with(fastrand::uppercase).take(6).collect();

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

            let existing_joined_instance = entity::prelude::UserSession::find()
                .filter(user_session::Column::SessionId.eq(session.id))
                .filter(user_session::Column::UserId.eq(user.0.id))
                .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
                .filter(user_session::Column::Proxy.is_null())
                .one(store.db())
                .await;

            match existing_joined_instance {
                Ok(Some(_)) => return StatusCode::OK.into_response(),
                Ok(None) => {}
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
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
        Ok(None) => (StatusCode::NOT_FOUND).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn set_session_proxy(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
    Json(body): Json<SetSessionProxyRequest>,
) -> impl IntoResponse {
    let store = &state.store;

    let session = match store.sessions().find_by_join_code(session_code).await {
        Ok(Some(session)) => session,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Session not found"})),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response();
        }
    };

    if session.status != SessionStatus::Open {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Session is not open"})),
        )
            .into_response();
    }

    let proxy_for = body
        .proxy_for
        .as_ref()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let existing_instances = match entity::prelude::UserSession::find()
        .filter(user_session::Column::SessionId.eq(session.id))
        .filter(user_session::Column::UserId.eq(user.0.id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .all(store.db())
        .await
    {
        Ok(instances) => instances,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response();
        }
    };

    let mut base_instances = existing_instances
        .iter()
        .filter(|instance| instance.proxy.is_none())
        .cloned()
        .collect::<Vec<_>>();
    let mut proxy_instances = existing_instances
        .iter()
        .filter(|instance| instance.proxy.is_some())
        .cloned()
        .collect::<Vec<_>>();

    if body.is_senator {
        if base_instances.is_empty() {
            let base = user_session::ActiveModel {
                user_id: Set(user.0.id),
                session_id: Set(session.id),
                join_left: Set(JoinLeft::Joined),
                ..Default::default()
            };

            if store.user_sessions().create(base).await.is_err() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to ensure senator vote instance"})),
                )
                    .into_response();
            }
        }
    } else {
        for base in &base_instances {
            if entity::prelude::UserSession::delete_by_id(base.id)
                .exec(store.db())
                .await
                .is_err()
            {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to remove non-senator vote instance"})),
                )
                    .into_response();
            }
        }
        base_instances.clear();
    }

    match proxy_for {
        Some(proxy_for_value) => {
            if let Some(proxy_instance) = proxy_instances.first() {
                let mut proxy_model: user_session::ActiveModel = proxy_instance.clone().into();
                proxy_model.proxy = Set(Some(proxy_for_value));

                if proxy_model.update(store.db()).await.is_err() {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to update proxy vote instance"})),
                    )
                        .into_response();
                }
            } else {
                let proxy_instance = user_session::ActiveModel {
                    user_id: Set(user.0.id),
                    session_id: Set(session.id),
                    proxy: Set(Some(proxy_for_value)),
                    join_left: Set(JoinLeft::Joined),
                    ..Default::default()
                };

                if store.user_sessions().create(proxy_instance).await.is_err() {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to add proxy vote instance"})),
                    )
                        .into_response();
                }
            }
        }
        None => {
            for proxy_instance in &proxy_instances {
                if entity::prelude::UserSession::delete_by_id(proxy_instance.id)
                    .exec(store.db())
                    .await
                    .is_err()
                {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to remove proxy vote instance"})),
                    )
                        .into_response();
                }
            }
            proxy_instances.clear();
        }
    }

    let final_instances = match entity::prelude::UserSession::find()
        .filter(user_session::Column::SessionId.eq(session.id))
        .filter(user_session::Column::UserId.eq(user.0.id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .all(store.db())
        .await
    {
        Ok(instances) => instances,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(SetSessionProxyResponse {
            vote_instance_count: final_instances.len(),
            is_senator: body.is_senator,
            has_proxy: final_instances
                .iter()
                .any(|instance| instance.proxy.is_some()),
        }),
    )
        .into_response()
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

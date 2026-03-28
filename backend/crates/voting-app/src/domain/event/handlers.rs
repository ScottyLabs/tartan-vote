use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{FixedOffset, Utc};
use entity::enums::{EventType, StatusOption};
use entity::event;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub name: String,
    #[serde(alias = "vote_type")]
    pub event_type: String,
    pub start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub end_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreateEventResponse {
    pub id: i32,
    pub name: String,
    pub event_type: EventType,
    pub status: StatusOption,
    pub start_time: chrono::DateTime<chrono::FixedOffset>,
}

pub async fn check_event(
    _user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> impl IntoResponse {
    let store = &state.store;

    let session = match store.sessions().find_by_join_code(session_code).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Session not found"})),
            )
                .into_response();
        }
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match store.events().find_active_by_session_id(session.id).await {
        Ok(Some(event)) => (
            StatusCode::OK,
            Json(json!({
                "active_event": {
                    "id": event.id,
                    "name": event.name,
                    "event_type": event.event_type,
                    "data": event.data,
                }
            })),
        )
            .into_response(),
        Ok(None) => (StatusCode::OK, Json(json!({ "active_event": null }))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn create_event(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
    Json(req): Json<CreateEventRequest>,
) -> impl IntoResponse {
    let store = &state.store;

    let parsed_event_type = match req.event_type.to_ascii_lowercase().as_str() {
        "motion" => EventType::Motion,
        "election" => EventType::Election,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid event_type; expected motion or election"})),
            )
                .into_response();
        }
    };

    // Find session by join code
    let session = match store
        .sessions()
        .find_by_join_code(session_code.clone())
        .await
    {
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

    let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
    let start_time = req.start_time.unwrap_or(now);

    let mut event_data = req.data.unwrap_or(serde_json::json!({}));
    if event_data.get("vote_type").is_none() {
        event_data["vote_type"] = match parsed_event_type {
            EventType::Motion => json!("motion"),
            EventType::Election => json!("election"),
        };
    }

    if let Some(visibility) = event_data
        .get("visibility")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
    {
        event_data["visibility"] = json!({"participants": visibility});
    }

    event_data["session_code"] = json!(session_code);

    let event_model = event::ActiveModel {
        name: Set(req.name.clone()),
        event_type: Set(parsed_event_type),
        status: Set(StatusOption::Active),
        start_time: Set(start_time),
        end_time: Set(req.end_time),
        data: Set(event_data),
        created_by_user_id: Set(user.0.id),
        session_id: Set(session.id),
        ..Default::default()
    };

    match store.events().create(event_model).await {
        Ok(event) => (
            StatusCode::CREATED,
            Json(CreateEventResponse {
                id: event.id,
                name: event.name,
                event_type: event.event_type,
                status: event.status,
                start_time: event.start_time,
            }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create event"})),
        )
            .into_response(),
    }
}

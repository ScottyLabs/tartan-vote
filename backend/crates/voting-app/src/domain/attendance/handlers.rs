use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::json;

#[axum::debug_handler]
pub async fn join(
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // TODO: replace with real auth middleware
    let _user_id = 1;

    let event = state
        .store
        .events()
        .find_active_by_session_code(&session_code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("No active event for session code '{session_code}'"),
            )
        })?;

    Ok(Json(json!({
        "event_id": event.id,
        "session_code": session_code,
        "message": "Successfully joined the session"
    })))
}

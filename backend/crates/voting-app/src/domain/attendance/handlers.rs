use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::json;
use voting_app_store::Store;

pub async fn get_attendance(
    store: &Store,
    session_code: &str,
) -> Result<Vec<entity::user::Model>, (StatusCode, String)> {
    let session = match store
        .sessions()
        .find_by_join_code(session_code.to_string())
        .await
    {
        Ok(Some(session)) => session,
        Ok(None) => {
            return Err((StatusCode::NOT_FOUND, "Session not found".to_string()));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch session: {err}"),
            ));
        }
    };

    store
        .user_sessions()
        .get_distinct_users(session.id)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch attendance: {err}"),
            )
        })
}

#[axum::debug_handler]
pub async fn attendance(
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let users = get_attendance(&state.store, &session_code).await?;

    let attendees: Vec<serde_json::Value> = users
        .into_iter()
        .map(|user| {
            json!({
                "id": user.id,
                "name": user.name,
                "andrew_id": user.andrew_id,
            })
        })
        .collect();

    let headcount = attendees.len();

    Ok(Json(json!({
        "session_code": session_code,
        "headcount": headcount,
        "attendees": attendees,
    })))
}

use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use entity::enums::JoinLeft;
use serde_json::json;
use std::collections::HashMap;
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
    _user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let session = state
        .store
        .sessions()
        .find_by_join_code(session_code.clone())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch session: {err}"),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Session not found".to_string()))?;

    let users = get_attendance(&state.store, &session_code).await?;
    let user_session_rows = state
        .store
        .user_sessions()
        .fetch_by_session_id(session.id)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch user sessions: {err}"),
            )
        })?;

    let mut proxy_map: HashMap<i32, Vec<String>> = HashMap::new();
    for row in user_session_rows {
        if row.join_left != JoinLeft::Joined {
            continue;
        }

        let Some(proxy_for) = row.proxy else {
            continue;
        };

        proxy_map.entry(row.user_id).or_default().push(proxy_for);
    }

    let attendees: Vec<serde_json::Value> = users
        .into_iter()
        .map(|user| {
            let proxy_for = proxy_map.remove(&user.id).unwrap_or_default();

            json!({
                "id": user.id,
                "name": user.name,
                "andrew_id": user.andrew_id,
                "is_proxy_holder": !proxy_for.is_empty(),
                "proxy_for": proxy_for,
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

use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{FixedOffset, Utc};
use entity::enums::StatusOption;
use entity::{motion, user_session};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMotionRequest {
    pub name: String,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub end_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(default)]
    #[schema(value_type = Object, nullable = true)]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateMotionResponse {
    pub id: i32,
    pub name: String,
    pub status: StatusOption,
    #[schema(value_type = String, format = DateTime)]
    pub start_time: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Deserialize)]
struct ProxyAssignmentInput {
    proxy_holder_user_id: i32,
    proxied_senator_user_id: i32,
}

fn parse_proxy_assignments(value: Option<&serde_json::Value>) -> Vec<ProxyAssignmentInput> {
    value
        .and_then(|v| v.as_array())
        .map(|array| {
            array
                .iter()
                .filter_map(|item| {
                    serde_json::from_value::<ProxyAssignmentInput>(item.clone()).ok()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn validate_proxy_assignments(
    proxy_enabled: bool,
    proxy_assignments: &[ProxyAssignmentInput],
) -> Result<(), &'static str> {
    if !proxy_enabled && !proxy_assignments.is_empty() {
        return Err("Proxy assignments provided, but proxy voting is disabled");
    }

    let mut seen_proxy_holders = HashSet::new();
    let mut seen_proxied_senators = HashSet::new();

    for assignment in proxy_assignments {
        if assignment.proxy_holder_user_id == assignment.proxied_senator_user_id {
            return Err("A user cannot proxy for themself");
        }

        if !seen_proxy_holders.insert(assignment.proxy_holder_user_id) {
            return Err("One participant may hold at most one proxy");
        }

        if !seen_proxied_senators.insert(assignment.proxied_senator_user_id) {
            return Err("A senator may only be proxied once per motion");
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EndMotionResponse {
    pub id: i32,
    pub status: StatusOption,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckMotionActiveMotion {
    pub id: i32,
    pub name: String,
    #[schema(value_type = Object)]
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckMotionResponse {
    pub active_motion: Option<CheckMotionActiveMotion>,
}

#[utoipa::path(
    get,
    path = "/motions/{session_code}/check",
    tag = "motions",
    params(
        ("session_code" = String, Path, description = "Session join code")
    ),
    responses(
        (status = 200, description = "Active motion for the session, or null", body = CheckMotionResponse),
        (status = 404, description = "Session not found"),
    )
)]
pub async fn check_motion(
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

    match store.motions().find_active_by_session_id(session.id).await {
        Ok(Some(motion)) => (
            StatusCode::OK,
            Json(CheckMotionResponse {
                active_motion: Some(CheckMotionActiveMotion {
                    id: motion.id,
                    name: motion.name,
                    data: motion.data,
                }),
            }),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::OK,
            Json(CheckMotionResponse {
                active_motion: None,
            }),
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/motions/create/{session_code}",
    tag = "motions",
    params(
        ("session_code" = String, Path, description = "Session join code")
    ),
    request_body = CreateMotionRequest,
    responses(
        (status = 201, description = "Motion created", body = CreateMotionResponse),
        (status = 400, description = "Invalid proxy configuration"),
        (status = 404, description = "Session not found"),
    )
)]
pub async fn create_motion(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
    Json(req): Json<CreateMotionRequest>,
) -> impl IntoResponse {
    let store = &state.store;

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

    let mut motion_data = req.data.unwrap_or(serde_json::json!({}));

    if let Some(visibility) = motion_data
        .get("visibility")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
    {
        motion_data["visibility"] = json!({"participants": visibility});
    }

    motion_data["session_code"] = json!(session_code);

    let proxy_assignments = parse_proxy_assignments(motion_data.get("proxy_assignments"));

    let motion_model = motion::ActiveModel {
        name: Set(req.name.clone()),
        status: Set(StatusOption::Active),
        start_time: Set(start_time),
        end_time: Set(req.end_time),
        data: Set(motion_data),
        created_by_user_id: Set(user.0.id),
        session_id: Set(session.id),
        ..Default::default()
    };

    let txn = match state.db.begin().await {
        Ok(txn) => txn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to start database transaction"})),
            )
                .into_response();
        }
    };

    let motion = match motion_model.insert(&txn).await {
        Ok(motion) => motion,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create motion"})),
            )
                .into_response();
        }
    };

    let proxy_enabled = motion.data["proxy"].as_bool().unwrap_or(false);
    if let Err(message) = validate_proxy_assignments(proxy_enabled, &proxy_assignments) {
        let _ = txn.rollback().await;
        return (StatusCode::BAD_REQUEST, Json(json!({"error": message}))).into_response();
    }

    let mut seen_proxy_targets = HashSet::new();

    for assignment in proxy_assignments {
        let holder = match entity::prelude::UserSession::find()
            .filter(user_session::Column::SessionId.eq(session.id))
            .filter(user_session::Column::UserId.eq(assignment.proxy_holder_user_id))
            .one(&txn)
            .await
        {
            Ok(Some(holder)) => holder,
            Ok(None) => {
                let _ = txn.rollback().await;
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Proxy holder must be in the session"})),
                )
                    .into_response();
            }
            Err(_) => {
                let _ = txn.rollback().await;
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error"})),
                )
                    .into_response();
            }
        };

        let proxied = match entity::prelude::UserSession::find()
            .filter(user_session::Column::SessionId.eq(session.id))
            .filter(user_session::Column::UserId.eq(assignment.proxied_senator_user_id))
            .one(&txn)
            .await
        {
            Ok(Some(proxied)) => proxied,
            Ok(None) => {
                let _ = txn.rollback().await;
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Proxied participant must be in the session"})),
                )
                    .into_response();
            }
            Err(_) => {
                let _ = txn.rollback().await;
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error"})),
                )
                    .into_response();
            }
        };

        let proxied_marker = proxied.user_id.to_string();
        if !seen_proxy_targets.insert(proxied_marker.clone()) {
            let _ = txn.rollback().await;
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "A senator may only be proxied once per motion"})),
            )
                .into_response();
        }

        let existing_target = match entity::prelude::UserSession::find()
            .filter(user_session::Column::SessionId.eq(session.id))
            .filter(user_session::Column::Proxy.eq(proxied_marker.clone()))
            .one(&txn)
            .await
        {
            Ok(existing) => existing,
            Err(_) => {
                let _ = txn.rollback().await;
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error"})),
                )
                    .into_response();
            }
        };

        if let Some(existing) = existing_target
            && existing.user_id != holder.user_id
        {
            let _ = txn.rollback().await;
            return (
                StatusCode::CONFLICT,
                Json(json!({"error": "A senator may only be proxied once per motion"})),
            )
                .into_response();
        }

        let mut holder_model: user_session::ActiveModel = holder.into();
        holder_model.proxy = Set(Some(proxied_marker));

        if holder_model.update(&txn).await.is_err() {
            let _ = txn.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create proxy assignment"})),
            )
                .into_response();
        }
    }

    if txn.commit().await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to commit motion transaction"})),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(CreateMotionResponse {
            id: motion.id,
            name: motion.name,
            status: motion.status,
            start_time: motion.start_time,
        }),
    )
        .into_response()
}

#[utoipa::path(
    post,
    path = "/motions/{id}/end",
    tag = "motions",
    params(
        ("id" = i32, Path, description = "Motion ID")
    ),
    responses(
        (status = 200, description = "Motion ended (or already inactive)", body = EndMotionResponse),
        (status = 403, description = "Only the motion creator can end this motion"),
        (status = 404, description = "Motion not found"),
    )
)]
pub async fn end_motion(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let store = &state.store;

    let motion = match store.motions().find_by_id(id).await {
        Ok(Some(motion)) => motion,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Motion not found"})),
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

    if motion.created_by_user_id != user.0.id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Only the motion creator can end this motion"})),
        )
            .into_response();
    }

    if motion.status == StatusOption::Inactive {
        return (
            StatusCode::OK,
            Json(EndMotionResponse {
                id: motion.id,
                status: motion.status,
            }),
        )
            .into_response();
    }

    let mut motion_to_update: motion::ActiveModel = motion.into_active_model();
    motion_to_update.status = Set(StatusOption::Inactive);

    match store.motions().update(motion_to_update).await {
        Ok(updated) => (
            StatusCode::OK,
            Json(EndMotionResponse {
                id: updated.id,
                status: updated.status,
            }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to end motion"})),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assignment(holder: i32, proxied: i32) -> ProxyAssignmentInput {
        ProxyAssignmentInput {
            proxy_holder_user_id: holder,
            proxied_senator_user_id: proxied,
        }
    }

    #[test]
    fn parse_proxy_assignments_skips_invalid_entries() {
        let input = json!([
            {
                "proxy_holder_user_id": 10,
                "proxied_senator_user_id": 20
            },
            {
                "proxy_holder_user_id": "bad",
                "proxied_senator_user_id": 21
            }
        ]);

        let parsed = parse_proxy_assignments(Some(&input));
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].proxy_holder_user_id, 10);
        assert_eq!(parsed[0].proxied_senator_user_id, 20);
    }

    #[test]
    fn validate_proxy_assignments_allows_valid_distinct_assignments() {
        let assignments = vec![assignment(10, 20), assignment(11, 21)];
        let result = validate_proxy_assignments(true, &assignments);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_proxy_assignments_rejects_when_disabled() {
        let assignments = vec![assignment(10, 20)];
        let result = validate_proxy_assignments(false, &assignments);
        assert_eq!(
            result.expect_err("should fail"),
            "Proxy assignments provided, but proxy voting is disabled"
        );
    }

    #[test]
    fn validate_proxy_assignments_rejects_self_proxy() {
        let assignments = vec![assignment(10, 10)];
        let result = validate_proxy_assignments(true, &assignments);
        assert_eq!(
            result.expect_err("should fail"),
            "A user cannot proxy for themself"
        );
    }

    #[test]
    fn validate_proxy_assignments_rejects_duplicate_holder() {
        let assignments = vec![assignment(10, 20), assignment(10, 21)];
        let result = validate_proxy_assignments(true, &assignments);
        assert_eq!(
            result.expect_err("should fail"),
            "One participant may hold at most one proxy"
        );
    }

    #[test]
    fn validate_proxy_assignments_rejects_duplicate_proxied_senator() {
        let assignments = vec![assignment(10, 20), assignment(11, 20)];
        let result = validate_proxy_assignments(true, &assignments);
        assert_eq!(
            result.expect_err("should fail"),
            "A senator may only be proxied once per motion"
        );
    }
}

use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use crate::core::error::AppError;
use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{FixedOffset, Utc};
use entity::enums::{EventType, StatusOption};
use entity::{event, user_session};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEventRequest {
    pub name: String,
    #[serde(alias = "vote_type")]
    pub event_type: String,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub end_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(default)]
    #[schema(value_type = Object, nullable = true)]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateEventResponse {
    pub id: i32,
    pub name: String,
    pub event_type: EventType,
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
            return Err("A senator may only be proxied once per event");
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EndEventResponse {
    pub id: i32,
    pub status: StatusOption,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckEventActiveEvent {
    pub id: i32,
    pub name: String,
    pub event_type: EventType,
    #[schema(value_type = Object)]
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckEventResponse {
    pub active_event: Option<CheckEventActiveEvent>,
}

#[utoipa::path(
    get,
    path = "/events/{session_code}/check",
    tag = "events",
    params(
        ("session_code" = String, Path, description = "Session join code")
    ),
    responses(
        (status = 200, description = "Active event for the session, or null", body = CheckEventResponse),
        (status = 404, description = "Session not found"),
    )
)]
pub async fn check_event(
    _user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let session = store
        .sessions()
        .find_by_join_code(session_code)
        .await?
        .ok_or_else(|| AppError::not_found("Session not found"))?;

    let active_event =
        if let Some(event) = store.events().find_active_by_session_id(session.id).await? {
            Some(CheckEventActiveEvent {
                id: event.id,
                name: event.name,
                event_type: event.event_type,
                data: event.data,
            })
        } else {
            None
        };

    Ok((StatusCode::OK, Json(CheckEventResponse { active_event })).into_response())
}

#[utoipa::path(
    post,
    path = "/events/create/{session_code}",
    tag = "events",
    params(
        ("session_code" = String, Path, description = "Session join code")
    ),
    request_body = CreateEventRequest,
    responses(
        (status = 201, description = "Event created", body = CreateEventResponse),
        (status = 400, description = "Invalid event_type or proxy configuration"),
        (status = 404, description = "Session not found"),
    )
)]
pub async fn create_event(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
    Json(req): Json<CreateEventRequest>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let parsed_event_type = match req.event_type.to_ascii_lowercase().as_str() {
        "motion" => EventType::Motion,
        "election" => EventType::Election,
        _ => {
            return Err(AppError::bad_request(
                "Invalid event_type; expected motion or election",
            ));
        }
    };

    let session = store
        .sessions()
        .find_by_join_code(session_code.clone())
        .await?
        .ok_or_else(|| AppError::not_found("Session not found"))?;

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

    let proxy_assignments = parse_proxy_assignments(event_data.get("proxy_assignments"));

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

    let txn = state
        .db
        .begin()
        .await
        .map_err(|_| AppError::internal("Failed to start database transaction"))?;

    let event = event_model
        .insert(&txn)
        .await
        .map_err(|_| AppError::internal("Failed to create event"))?;

    let proxy_enabled = event.data["proxy"].as_bool().unwrap_or(false);
    if let Err(message) = validate_proxy_assignments(proxy_enabled, &proxy_assignments) {
        let _ = txn.rollback().await;
        return Err(AppError::bad_request(message));
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
                return Err(AppError::bad_request("Proxy holder must be in the session"));
            }
            Err(_) => {
                let _ = txn.rollback().await;
                return Err(AppError::internal("Database error"));
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
                return Err(AppError::bad_request(
                    "Proxied participant must be in the session",
                ));
            }
            Err(_) => {
                let _ = txn.rollback().await;
                return Err(AppError::internal("Database error"));
            }
        };

        let proxied_marker = proxied.user_id.to_string();
        if !seen_proxy_targets.insert(proxied_marker.clone()) {
            let _ = txn.rollback().await;
            return Err(AppError::bad_request(
                "A senator may only be proxied once per event",
            ));
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
                return Err(AppError::internal("Database error"));
            }
        };

        if let Some(existing) = existing_target
            && existing.user_id != holder.user_id
        {
            let _ = txn.rollback().await;
            return Err(AppError::conflict(
                "A senator may only be proxied once per event",
            ));
        }

        let mut holder_model: user_session::ActiveModel = holder.into();
        holder_model.proxy = Set(Some(proxied_marker));

        if holder_model.update(&txn).await.is_err() {
            let _ = txn.rollback().await;
            return Err(AppError::internal("Failed to create proxy assignment"));
        }
    }

    if txn.commit().await.is_err() {
        return Err(AppError::internal("Failed to commit event transaction"));
    }

    Ok((
        StatusCode::CREATED,
        Json(CreateEventResponse {
            id: event.id,
            name: event.name,
            event_type: event.event_type,
            status: event.status,
            start_time: event.start_time,
        }),
    )
        .into_response())
}

#[utoipa::path(
    post,
    path = "/events/{id}/end",
    tag = "events",
    params(
        ("id" = i32, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event ended (or already inactive)", body = EndEventResponse),
        (status = 403, description = "Only the event creator can end this event"),
        (status = 404, description = "Event not found"),
    )
)]
pub async fn end_event(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let event = store
        .events()
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    if event.created_by_user_id != user.0.id {
        return Err(AppError::forbidden(
            "Only the event creator can end this event",
        ));
    }

    if event.status == StatusOption::Inactive {
        return Ok((
            StatusCode::OK,
            Json(EndEventResponse {
                id: event.id,
                status: event.status,
            }),
        )
            .into_response());
    }

    let mut event_to_update: event::ActiveModel = event.into_active_model();
    event_to_update.status = Set(StatusOption::Inactive);

    let updated = store
        .events()
        .update(event_to_update)
        .await
        .map_err(|_| AppError::internal("Failed to end event"))?;

    Ok((
        StatusCode::OK,
        Json(EndEventResponse {
            id: updated.id,
            status: updated.status,
        }),
    )
        .into_response())
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
            "A senator may only be proxied once per event"
        );
    }
}

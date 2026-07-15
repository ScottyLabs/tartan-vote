use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use crate::core::error::AppError;
use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use entity::enums::{JoinLeft, SessionStatus};
use entity::{session, user_session};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::Deserialize;
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

pub async fn create_session(
    user: SyncedUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let session_code = petname::petname(2, "-").expect("Failed to generate session code");

    // logic should be implemented to verify that the join code is actually unique
    // but as of right now this is low priority because 36^6 > 2 billion
    // TODO: database logic to force uniqueness
    let session = session::ActiveModel {
        join_code: Set(session_code),
        status: Set(SessionStatus::Open),
        created_by_user_id: Set(user.0.id),
        ..Default::default()
    };
    let session = store.sessions().create(session).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateSessionResponse {
            session_code: session.join_code,
        }),
    )
        .into_response())
}

pub async fn status_session(
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

    Ok((
        StatusCode::OK,
        Json(json!({ "session_ended": session.status == SessionStatus::Closed })),
    )
        .into_response())
}

pub async fn join_session(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let session = store
        .sessions()
        .find_by_join_code(session_code)
        .await?
        .ok_or_else(|| AppError::not_found("Session not found"))?;

    if session.status != SessionStatus::Open {
        return Err(AppError::forbidden("Session is not open"));
    }

    let existing_joined_instance = entity::prelude::UserSession::find()
        .filter(user_session::Column::SessionId.eq(session.id))
        .filter(user_session::Column::UserId.eq(user.0.id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .filter(user_session::Column::Proxy.is_null())
        .one(store.db())
        .await?;

    if existing_joined_instance.is_some() {
        return Ok(StatusCode::OK.into_response());
    }

    let new_user_session = user_session::ActiveModel {
        user_id: Set(user.0.id),
        session_id: Set(session.id),
        join_left: Set(JoinLeft::Joined),
        ..Default::default()
    };

    store.user_sessions().create(new_user_session).await?;
    Ok(StatusCode::OK.into_response())
}

fn normalize_proxy_for(proxy_for: Option<&str>) -> Option<String> {
    proxy_for
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub async fn set_session_proxy(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
    Json(body): Json<SetSessionProxyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let session = store
        .sessions()
        .find_by_join_code(session_code)
        .await?
        .ok_or_else(|| AppError::not_found("Session not found"))?;

    if session.status != SessionStatus::Open {
        return Err(AppError::forbidden("Session is not open"));
    }

    let proxy_for = normalize_proxy_for(body.proxy_for.as_deref());

    let existing_instances = entity::prelude::UserSession::find()
        .filter(user_session::Column::SessionId.eq(session.id))
        .filter(user_session::Column::UserId.eq(user.0.id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .all(store.db())
        .await?;

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

            store
                .user_sessions()
                .create(base)
                .await
                .map_err(|_| AppError::internal("Failed to ensure senator vote instance"))?;
        }
    } else {
        for base in &base_instances {
            entity::prelude::UserSession::delete_by_id(base.id)
                .exec(store.db())
                .await
                .map_err(|_| AppError::internal("Failed to remove non-senator vote instance"))?;
        }
        base_instances.clear();
    }

    match proxy_for {
        Some(proxy_for_value) => {
            if let Some(proxy_instance) = proxy_instances.first() {
                let mut proxy_model: user_session::ActiveModel = proxy_instance.clone().into();
                proxy_model.proxy = Set(Some(proxy_for_value));

                proxy_model
                    .update(store.db())
                    .await
                    .map_err(|_| AppError::internal("Failed to update proxy vote instance"))?;
            } else {
                let proxy_instance = user_session::ActiveModel {
                    user_id: Set(user.0.id),
                    session_id: Set(session.id),
                    proxy: Set(Some(proxy_for_value)),
                    join_left: Set(JoinLeft::Joined),
                    ..Default::default()
                };

                store
                    .user_sessions()
                    .create(proxy_instance)
                    .await
                    .map_err(|_| AppError::internal("Failed to add proxy vote instance"))?;
            }
        }
        None => {
            for proxy_instance in &proxy_instances {
                entity::prelude::UserSession::delete_by_id(proxy_instance.id)
                    .exec(store.db())
                    .await
                    .map_err(|_| AppError::internal("Failed to remove proxy vote instance"))?;
            }
            proxy_instances.clear();
        }
    }

    let final_instances = entity::prelude::UserSession::find()
        .filter(user_session::Column::SessionId.eq(session.id))
        .filter(user_session::Column::UserId.eq(user.0.id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .all(store.db())
        .await?;

    Ok((
        StatusCode::OK,
        Json(SetSessionProxyResponse {
            vote_instance_count: final_instances.len(),
            is_senator: body.is_senator,
            has_proxy: final_instances
                .iter()
                .any(|instance| instance.proxy.is_some()),
        }),
    )
        .into_response())
}

pub async fn end_session(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let session = store
        .sessions()
        .find_by_join_code(session_code.clone())
        .await?
        .ok_or_else(|| AppError::not_found("Session not found"))?;

    if session.created_by_user_id != user.0.id {
        return Err(AppError::forbidden(
            "Only the session creator can end this session",
        ));
    }

    if session.status == SessionStatus::Closed {
        return Ok((
            StatusCode::OK,
            Json(EndSessionResponse {
                session_code,
                status: SessionStatus::Closed,
            }),
        )
            .into_response());
    }

    let mut session_to_update = session.into_active_model();
    session_to_update.status = Set(SessionStatus::Closed);

    let updated = store.sessions().update(session_to_update).await?;

    Ok((
        StatusCode::OK,
        Json(EndSessionResponse {
            session_code: updated.join_code,
            status: updated.status,
        }),
    )
        .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::{Path, State};
    use chrono::Utc;
    use entity::enums::JoinLeft;
    use entity::{session, user, user_session};
    use sea_orm::{DatabaseBackend, MockDatabase};
    use std::sync::Arc;
    use voting_app_store::Store;

    fn test_user(id: i32) -> SyncedUser {
        SyncedUser(Arc::new(user::Model {
            id,
            name: "Test User".to_string(),
            andrew_id: "testuser".to_string(),
            oidc_subject: "test-oidc-subject".to_string(),
            created_at: Utc::now().fixed_offset(),
        }))
    }

    fn make_state(db: sea_orm::DatabaseConnection) -> AppState {
        AppState {
            store: Store::new(db.clone()),
            db,
            config: crate::config::Config {
                database_url: String::new(),
                bind_addr: String::new(),
                sentry_dsn: None,
            },
        }
    }

    async fn axum_to_json(response: axum::response::Response) -> serde_json::Value {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    fn make_session(
        id: i32,
        join_code: &str,
        status: SessionStatus,
        created_by: i32,
    ) -> session::Model {
        session::Model {
            id,
            join_code: join_code.to_string(),
            status,
            created_by_user_id: created_by,
        }
    }

    fn make_user_session(
        id: i32,
        user_id: i32,
        session_id: i32,
        proxy: Option<String>,
    ) -> user_session::Model {
        user_session::Model {
            id,
            user_id,
            session_id,
            proxy,
            join_left: JoinLeft::Joined,
            timestamp: Utc::now().fixed_offset(),
        }
    }

    #[test]
    fn normalize_proxy_for_returns_none_when_absent() {
        assert_eq!(normalize_proxy_for(None), None);
    }

    #[test]
    fn normalize_proxy_for_returns_none_for_empty_string() {
        assert_eq!(normalize_proxy_for(Some("")), None);
    }

    #[test]
    fn normalize_proxy_for_returns_none_for_whitespace_only() {
        assert_eq!(normalize_proxy_for(Some("   ")), None);
    }

    #[test]
    fn normalize_proxy_for_returns_trimmed_value() {
        assert_eq!(
            normalize_proxy_for(Some("  alice  ")),
            Some("alice".to_string())
        );
    }

    #[test]
    fn normalize_proxy_for_returns_value_unchanged_when_no_padding() {
        assert_eq!(
            normalize_proxy_for(Some("alice")),
            Some("alice".to_string())
        );
    }

    // fn status_session

    #[tokio::test]
    async fn status_session_reports_not_ended_for_open_session() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![make_session(1, "ABC123", SessionStatus::Open, 1)]])
            .into_connection();

        let response = status_session(
            test_user(1),
            State(make_state(db)),
            Path("ABC123".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        let json = axum_to_json(response).await;
        assert_eq!(json["session_ended"], false);
    }

    #[tokio::test]
    async fn status_session_reports_ended_for_closed_session() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![make_session(1, "ABC123", SessionStatus::Closed, 1)]])
            .into_connection();

        let response = status_session(
            test_user(1),
            State(make_state(db)),
            Path("ABC123".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        let json = axum_to_json(response).await;
        assert_eq!(json["session_ended"], true);
    }

    #[tokio::test]
    async fn status_session_returns_404_when_not_found() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![] as Vec<session::Model>])
            .into_connection();

        let response = status_session(
            test_user(1),
            State(make_state(db)),
            Path("NOPE00".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // fn join_session

    #[tokio::test]
    async fn join_session_succeeds_for_new_participant() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![make_session(1, "ABC123", SessionStatus::Open, 99)]])
            .append_query_results([vec![] as Vec<user_session::Model>])
            .append_query_results([vec![make_user_session(10, 1, 1, None)]])
            .into_connection();

        let response = join_session(
            test_user(1),
            State(make_state(db)),
            Path("ABC123".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn join_session_is_idempotent_when_already_joined() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![make_session(1, "ABC123", SessionStatus::Open, 99)]])
            .append_query_results([vec![make_user_session(10, 1, 1, None)]])
            .into_connection();

        let response = join_session(
            test_user(1),
            State(make_state(db)),
            Path("ABC123".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn join_session_rejects_closed_session() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![make_session(1, "ABC123", SessionStatus::Closed, 99)]])
            .into_connection();

        let response = join_session(
            test_user(1),
            State(make_state(db)),
            Path("ABC123".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn join_session_returns_404_when_not_found() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![] as Vec<session::Model>])
            .into_connection();

        let response = join_session(
            test_user(1),
            State(make_state(db)),
            Path("NOPE00".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    // fn end_session

    #[tokio::test]
    async fn end_session_closes_open_session_for_creator() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                vec![make_session(1, "ABC123", SessionStatus::Open, 1)],
                vec![make_session(1, "ABC123", SessionStatus::Closed, 1)],
            ])
            .into_connection();

        let response = end_session(
            test_user(1),
            State(make_state(db)),
            Path("ABC123".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        let json = axum_to_json(response).await;
        assert_eq!(json["session_code"], "ABC123");
    }

    #[tokio::test]
    async fn end_session_rejects_non_creator() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![make_session(1, "ABC123", SessionStatus::Open, 99)]])
            .into_connection();

        let response = end_session(
            test_user(1),
            State(make_state(db)),
            Path("ABC123".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn end_session_is_idempotent_when_already_closed() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![make_session(1, "ABC123", SessionStatus::Closed, 1)]])
            .into_connection();

        let response = end_session(
            test_user(1),
            State(make_state(db)),
            Path("ABC123".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        let json = axum_to_json(response).await;
        assert_eq!(json["session_code"], "ABC123");
    }

    #[tokio::test]
    async fn end_session_returns_404_when_not_found() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![] as Vec<session::Model>])
            .into_connection();

        let response = end_session(
            test_user(1),
            State(make_state(db)),
            Path("NOPE00".to_string()),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

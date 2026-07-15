use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use crate::core::error::AppError;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use entity::enums::{JoinLeft, StatusOption};
use entity::{prelude::User, prelude::UserSession, prelude::Vote, user_session, vote};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CastVoteRequest {
    pub vote_response: Vec<String>,
    pub voter_instance_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AssignProxyRequest {
    pub proxy_holder_user_id: i32,
    pub proxied_senator_user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct AssignProxyResponse {
    pub voter_instance_id: i32,
    pub proxy_holder_user_id: i32,
    pub proxied_senator_user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct VoteInstance {
    pub voter_instance_id: i32,
    pub is_proxy: bool,
    pub proxy_for_user_id: Option<i32>,
    pub proxy_for_name: Option<String>,
    pub has_voted: bool,
}

#[derive(Debug, Serialize)]
pub struct ProxyAssignment {
    pub voter_instance_id: i32,
    pub proxy_holder_user_id: i32,
    pub proxy_holder_name: Option<String>,
    pub proxied_senator_user_id: i32,
    pub proxied_senator_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VoteExportRecord {
    pub voter_instance_id: i32,
    pub cast_time: String,
    pub voter_user_id: i32,
    pub voter_name: Option<String>,
    pub is_proxy: bool,
    pub proxied_senator_user_id: Option<i32>,
    pub proxied_senator_name: Option<String>,
    pub vote_response: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct EventExportResponse {
    pub event_id: i32,
    pub event_name: String,
    pub proxy_assignments: Vec<ProxyAssignment>,
    pub totals: MotionResults,
    pub votes: Vec<VoteExportRecord>,
}

#[derive(Debug, Serialize)]
pub struct MotionResults {
    pub pass: u32,
    pub reject: u32,
    pub abstain: u32,
    pub total: u32,
    pub threshold: f64,
    pub passed: bool,
}

#[derive(Debug, Serialize)]
pub struct ElectionOptionResult {
    pub label: String,
    pub count: u32,
    pub percent: u32,
}

#[derive(Debug, Serialize)]
pub struct ElectionResults {
    pub vote_type: String,
    pub total: u32,
    pub options: Vec<ElectionOptionResult>,
}

fn parse_proxy_for_user_id(proxy: &Option<String>) -> Option<i32> {
    proxy.as_ref().and_then(|value| value.parse::<i32>().ok())
}

async fn user_name_by_id(store: &voting_app_store::Store, user_id: i32) -> Option<String> {
    match User::find_by_id(user_id).one(store.db()).await {
        Ok(Some(user)) => Some(user.name),
        _ => None,
    }
}

fn compute_motion_totals(vote_records: &[VoteExportRecord], threshold: f64) -> MotionResults {
    let mut pass = 0u32;
    let mut reject = 0u32;
    let mut abstain = 0u32;

    let classify_motion_vote = |value: &str| match value.trim().to_ascii_lowercase().as_str() {
        "pass" | "yes" | "yay" | "approve" | "approved" | "for" => Some("pass"),
        "reject" | "no" | "nay" | "deny" | "denied" | "against" => Some("reject"),
        "abstain" | "abstained" | "abstention" => Some("abstain"),
        _ => None,
    };

    for record in vote_records {
        let response = record
            .vote_response
            .first()
            .map(|value| value.to_string())
            .unwrap_or_default();

        match classify_motion_vote(&response) {
            Some("pass") => pass += 1,
            Some("reject") => reject += 1,
            Some("abstain") => abstain += 1,
            _ => {}
        }
    }

    let total = pass + reject + abstain;
    let denominator = pass + reject;
    let passed = denominator > 0 && (pass as f64 / denominator as f64) > threshold;

    MotionResults {
        pass,
        reject,
        abstain,
        total,
        threshold,
        passed,
    }
}

fn compute_election_totals(
    vote_records: &[VoteExportRecord],
    vote_options: &[String],
) -> ElectionResults {
    let mut counts: HashMap<String, u32> = vote_options
        .iter()
        .map(|option| (option.clone(), 0u32))
        .collect();

    for record in vote_records {
        let response = record.vote_response.first().cloned().unwrap_or_default();
        if let Some(count) = counts.get_mut(&response) {
            *count += 1;
        }
    }

    let total: u32 = counts.values().sum();

    let options = vote_options
        .iter()
        .map(|label| {
            let count = *counts.get(label).unwrap_or(&0);
            let percent = if total > 0 {
                ((count as f64 / total as f64) * 100.0).round() as u32
            } else {
                0
            };

            ElectionOptionResult {
                label: label.clone(),
                count,
                percent,
            }
        })
        .collect();

    ElectionResults {
        vote_type: "election".to_string(),
        total,
        options,
    }
}

fn select_voter_instance(
    voter_instances: &[user_session::Model],
    requested_instance_id: Option<i32>,
) -> Result<user_session::Model, &'static str> {
    if let Some(requested_id) = requested_instance_id {
        voter_instances
            .iter()
            .find(|instance| instance.id == requested_id)
            .cloned()
            .ok_or("Invalid voter_instance_id for this user")
    } else if voter_instances.len() == 1 {
        Ok(voter_instances[0].clone())
    } else {
        Err("Multiple vote instances available; voter_instance_id is required")
    }
}

pub async fn cast_vote(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(event_id): Path<i32>,
    Json(body): Json<CastVoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let event = store
        .events()
        .find_by_id(event_id)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    let event_data = event.data.clone();
    let vote_type = event_data["vote_type"].as_str().unwrap_or("");

    if vote_type != "motion" && vote_type != "election" {
        return Err(AppError::bad_request("Event is not a motion"));
    }

    let voter_instances = UserSession::find()
        .filter(user_session::Column::SessionId.eq(event.session_id))
        .filter(user_session::Column::UserId.eq(user.0.id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .all(store.db())
        .await?;

    if voter_instances.is_empty() {
        return Err(AppError::forbidden(
            "User is not eligible to vote in this event",
        ));
    }

    let selected_voter = select_voter_instance(&voter_instances, body.voter_instance_id)
        .map_err(AppError::bad_request)?;

    if event.status != StatusOption::Active {
        return Err(AppError::bad_request("Event is not open"));
    }

    if selected_voter.proxy.is_some() && !event_data["proxy"].as_bool().unwrap_or(false) {
        return Err(AppError::bad_request(
            "Proxy voting is not allowed for this event",
        ));
    }

    if body.vote_response.is_empty() {
        return Err(AppError::bad_request("vote_response cannot be empty"));
    }

    let vote_options: Vec<String> = event_data["vote_options"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    if !vote_options.contains(&body.vote_response[0]) {
        return Err(AppError::bad_request("Invalid vote option"));
    }

    if store
        .votes()
        .find_by_event_and_user_session(event.id, selected_voter.id)
        .await?
        .is_some()
    {
        return Err(AppError::conflict(
            "This vote instance has already cast a vote",
        ));
    }

    let new_vote = vote::ActiveModel {
        event_id: Set(event.id),
        user_session_id: Set(selected_voter.id),
        cast_time: Set(Utc::now().into()),
        data: Set(json!({
            "vote_type": vote_type,
            "proxy": selected_voter.proxy.is_some(),
            "proxy_for_user_id": parse_proxy_for_user_id(&selected_voter.proxy),
            "proxy_for": selected_voter.proxy,
            "vote_response": body.vote_response,
        })),
        ..Default::default()
    };

    let created_vote = store
        .votes()
        .create(new_vote)
        .await
        .map_err(|_| AppError::internal("Failed to cast vote"))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "Vote cast successfully",
            "voter_instance_id": selected_voter.id,
            "vote_id": created_vote.id
        })),
    )
        .into_response())
}

pub async fn get_motion_results(
    _user: SyncedUser,
    State(state): State<AppState>,
    Path(event_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let store = state.store;

    let event = store
        .events()
        .find_by_id(event_id)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    let event_data = event.data.clone();
    let vote_type = event_data["vote_type"].as_str().unwrap_or("");

    if vote_type != "motion" && vote_type != "election" {
        return Err(AppError::bad_request("Event is not a supported vote type"));
    }

    //Place holder for when we figure the visibility out
    let visibility = event_data["visibility"]["participants"]
        .as_str()
        .unwrap_or("");
    if visibility == "hidden_until_release" && event.status != StatusOption::Inactive {
        return Err(AppError::forbidden("Results are not yet available"));
    }

    let votes = Vote::find()
        .find_also_related(user_session::Entity)
        .filter(vote::Column::EventId.eq(event_id))
        .all(store.db())
        .await?;

    let threshold = event_data["threshold"].as_f64().unwrap_or(0.5);
    let vote_options: Vec<String> = event_data["vote_options"]
        .as_array()
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(ToOwned::to_owned))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut export_records = Vec::new();

    for (vote, related_user_session) in votes {
        let Some(related_user_session) = related_user_session else {
            continue;
        };

        let vote_response = vote
            .data
            .get("vote_response")
            .and_then(|value| value.as_array())
            .map(|responses| {
                responses
                    .iter()
                    .filter_map(|response| response.as_str().map(ToOwned::to_owned))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        export_records.push(VoteExportRecord {
            voter_instance_id: related_user_session.id,
            cast_time: vote.cast_time.to_rfc3339(),
            voter_user_id: related_user_session.user_id,
            voter_name: None,
            is_proxy: related_user_session.proxy.is_some(),
            proxied_senator_user_id: parse_proxy_for_user_id(&related_user_session.proxy),
            proxied_senator_name: None,
            vote_response,
        });
    }

    if vote_type == "motion" {
        let motion_results = compute_motion_totals(&export_records, threshold);
        return Ok((StatusCode::OK, Json(json!(motion_results))).into_response());
    }

    let election_results = compute_election_totals(&export_records, &vote_options);
    Ok((StatusCode::OK, Json(json!(election_results))).into_response())
}

pub async fn assign_proxy(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(event_id): Path<i32>,
    Json(body): Json<AssignProxyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    if body.proxy_holder_user_id == body.proxied_senator_user_id {
        return Err(AppError::bad_request("A user cannot proxy for themself"));
    }

    let event = store
        .events()
        .find_by_id(event_id)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    if user.0.id != event.created_by_user_id {
        return Err(AppError::forbidden(
            "Only the event host may assign proxies",
        ));
    }

    if !event.data["proxy"].as_bool().unwrap_or(false) {
        return Err(AppError::bad_request(
            "Proxy voting is not enabled for this event",
        ));
    }

    let proxy_holder = UserSession::find()
        .filter(user_session::Column::SessionId.eq(event.session_id))
        .filter(user_session::Column::UserId.eq(body.proxy_holder_user_id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .one(store.db())
        .await?;

    let Some(proxy_holder) = proxy_holder else {
        return Err(AppError::bad_request("Proxy holder must be in the session"));
    };

    if proxy_holder.proxy.is_some() {
        return Err(AppError::conflict(
            "One participant may hold at most one proxy",
        ));
    }

    let proxied_marker = body.proxied_senator_user_id.to_string();
    let proxied_participant = UserSession::find()
        .filter(user_session::Column::SessionId.eq(event.session_id))
        .filter(user_session::Column::UserId.eq(body.proxied_senator_user_id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .one(store.db())
        .await?;

    if proxied_participant.is_none() {
        return Err(AppError::bad_request(
            "Proxied participant must be in the session",
        ));
    }

    let already_proxied = UserSession::find()
        .filter(user_session::Column::SessionId.eq(event.session_id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .filter(user_session::Column::Proxy.eq(proxied_marker.clone()))
        .one(store.db())
        .await?;

    if let Some(existing) = already_proxied
        && existing.user_id != body.proxy_holder_user_id
    {
        return Err(AppError::conflict(
            "This senator already has a proxy assignment",
        ));
    }

    let mut holder_model: user_session::ActiveModel = proxy_holder.into();
    holder_model.proxy = Set(Some(proxied_marker));

    let updated = holder_model
        .update(store.db())
        .await
        .map_err(|_| AppError::internal("Failed to create proxy assignment"))?;

    Ok((
        StatusCode::CREATED,
        Json(json!(AssignProxyResponse {
            voter_instance_id: updated.id,
            proxy_holder_user_id: body.proxy_holder_user_id,
            proxied_senator_user_id: body.proxied_senator_user_id,
        })),
    )
        .into_response())
}

pub async fn list_proxy_assignments(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(event_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let event = store
        .events()
        .find_by_id(event_id)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    if user.0.id != event.created_by_user_id {
        return Err(AppError::forbidden(
            "Only the event host may view proxy assignments",
        ));
    }

    let proxy_voters = UserSession::find()
        .filter(user_session::Column::SessionId.eq(event.session_id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .filter(user_session::Column::Proxy.is_not_null())
        .all(store.db())
        .await?;

    let mut assignments = Vec::new();
    for instance in proxy_voters {
        let proxied_senator_user_id = match parse_proxy_for_user_id(&instance.proxy) {
            Some(user_id) => user_id,
            None => continue,
        };

        assignments.push(ProxyAssignment {
            voter_instance_id: instance.id,
            proxy_holder_user_id: instance.user_id,
            proxy_holder_name: user_name_by_id(store, instance.user_id).await,
            proxied_senator_user_id,
            proxied_senator_name: user_name_by_id(store, proxied_senator_user_id).await,
        });
    }

    Ok((StatusCode::OK, Json(assignments)).into_response())
}

pub async fn get_vote_instances(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(event_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let event = store
        .events()
        .find_by_id(event_id)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    let voter_instances = UserSession::find()
        .filter(user_session::Column::SessionId.eq(event.session_id))
        .filter(user_session::Column::UserId.eq(user.0.id))
        .filter(user_session::Column::JoinLeft.eq(JoinLeft::Joined))
        .all(store.db())
        .await?;

    let mut response = Vec::new();
    for instance in voter_instances {
        let proxy_for_user_id = parse_proxy_for_user_id(&instance.proxy);
        let has_voted = store
            .votes()
            .find_by_event_and_user_session(event.id, instance.id)
            .await
            .ok()
            .flatten()
            .is_some();

        response.push(VoteInstance {
            voter_instance_id: instance.id,
            is_proxy: instance.proxy.is_some(),
            proxy_for_user_id,
            proxy_for_name: match proxy_for_user_id {
                Some(proxy_for_id) => user_name_by_id(store, proxy_for_id)
                    .await
                    .or_else(|| instance.proxy.clone()),
                None => instance.proxy.clone(),
            },
            has_voted,
        });
    }

    Ok((StatusCode::OK, Json(response)).into_response())
}

pub async fn export_event_results(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(event_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let store = &state.store;

    let event = store
        .events()
        .find_by_id(event_id)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    if user.0.id != event.created_by_user_id {
        return Err(AppError::forbidden(
            "Only the event host may export results",
        ));
    }

    let vote_rows = Vote::find()
        .find_also_related(user_session::Entity)
        .filter(vote::Column::EventId.eq(event_id))
        .all(store.db())
        .await?;

    let mut user_name_cache: HashMap<i32, Option<String>> = HashMap::new();
    let mut votes = Vec::new();

    for (vote_row, related_user_session) in vote_rows {
        let Some(voter_row) = related_user_session else {
            continue;
        };

        let voter_name = if let Some(cached) = user_name_cache.get(&voter_row.user_id) {
            cached.clone()
        } else {
            let name = user_name_by_id(store, voter_row.user_id).await;
            user_name_cache.insert(voter_row.user_id, name.clone());
            name
        };

        let proxied_senator_user_id = parse_proxy_for_user_id(&voter_row.proxy);
        let proxied_senator_name = if let Some(proxy_user_id) = proxied_senator_user_id {
            if let Some(cached) = user_name_cache.get(&proxy_user_id) {
                cached.clone()
            } else {
                let name = user_name_by_id(store, proxy_user_id).await;
                user_name_cache.insert(proxy_user_id, name.clone());
                name
            }
        } else {
            None
        };

        let vote_response = vote_row
            .data
            .get("vote_response")
            .and_then(|value| value.as_array())
            .map(|responses| {
                responses
                    .iter()
                    .filter_map(|response| response.as_str().map(ToOwned::to_owned))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        votes.push(VoteExportRecord {
            voter_instance_id: voter_row.id,
            cast_time: vote_row.cast_time.to_rfc3339(),
            voter_user_id: voter_row.user_id,
            voter_name,
            is_proxy: voter_row.proxy.is_some(),
            proxied_senator_user_id,
            proxied_senator_name,
            vote_response,
        });
    }

    let proxy_assignments = votes
        .iter()
        .filter(|record| record.is_proxy)
        .filter_map(|record| {
            record
                .proxied_senator_user_id
                .map(|proxied_id| ProxyAssignment {
                    voter_instance_id: record.voter_instance_id,
                    proxy_holder_user_id: record.voter_user_id,
                    proxy_holder_name: record.voter_name.clone(),
                    proxied_senator_user_id: proxied_id,
                    proxied_senator_name: record.proxied_senator_name.clone(),
                })
        })
        .collect::<Vec<_>>();

    let threshold = event.data["threshold"].as_f64().unwrap_or(0.5);
    let totals = compute_motion_totals(&votes, threshold);

    Ok((
        StatusCode::OK,
        Json(EventExportResponse {
            event_id: event.id,
            event_name: event.name,
            proxy_assignments,
            totals,
            votes,
        }),
    )
        .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::FixedOffset;
    use entity::enums::JoinLeft;

    fn build_vote_record(response: &str) -> VoteExportRecord {
        VoteExportRecord {
            voter_instance_id: 1,
            cast_time: "2026-01-01T00:00:00Z".to_string(),
            voter_user_id: 1,
            voter_name: Some("Test User".to_string()),
            is_proxy: false,
            proxied_senator_user_id: None,
            proxied_senator_name: None,
            vote_response: vec![response.to_string()],
        }
    }

    #[test]
    fn parse_proxy_for_user_id_parses_valid_numeric_proxy() {
        let value = Some("42".to_string());
        assert_eq!(parse_proxy_for_user_id(&value), Some(42));
    }

    #[test]
    fn parse_proxy_for_user_id_returns_none_for_invalid_proxy() {
        let value = Some("senator-abc".to_string());
        assert_eq!(parse_proxy_for_user_id(&value), None);
    }

    #[test]
    fn compute_motion_totals_counts_votes_and_threshold() {
        let vote_records = vec![
            build_vote_record("Pass"),
            build_vote_record("pass"),
            build_vote_record("Reject"),
            build_vote_record("Abstain"),
        ];

        let totals = compute_motion_totals(&vote_records, 0.5);

        assert_eq!(totals.pass, 2);
        assert_eq!(totals.reject, 1);
        assert_eq!(totals.abstain, 1);
        assert_eq!(totals.total, 4);
        assert!(totals.passed);
    }

    #[test]
    fn compute_motion_totals_does_not_pass_without_pass_reject_denominator() {
        let vote_records = vec![build_vote_record("Abstain")];
        let totals = compute_motion_totals(&vote_records, 0.5);

        assert_eq!(totals.pass, 0);
        assert_eq!(totals.reject, 0);
        assert_eq!(totals.abstain, 1);
        assert!(!totals.passed);
    }

    #[test]
    fn compute_motion_totals_counts_yes_no_labels() {
        let vote_records = vec![
            build_vote_record("Yes"),
            build_vote_record("No"),
            build_vote_record("Abstain"),
        ];

        let totals = compute_motion_totals(&vote_records, 0.5);

        assert_eq!(totals.pass, 1);
        assert_eq!(totals.reject, 1);
        assert_eq!(totals.abstain, 1);
        assert_eq!(totals.total, 3);
    }

    #[test]
    fn select_voter_instance_returns_single_instance_without_id() {
        let voters = vec![user_session::Model {
            id: 11,
            user_id: 22,
            session_id: 1,
            proxy: None,
            join_left: JoinLeft::Joined,
            timestamp: Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
        }];

        let selected = select_voter_instance(&voters, None).expect("instance should be selected");
        assert_eq!(selected.id, 11);
    }

    #[test]
    fn select_voter_instance_requires_id_for_multiple_instances() {
        let voters = vec![
            user_session::Model {
                id: 11,
                user_id: 22,
                session_id: 1,
                proxy: None,
                join_left: JoinLeft::Joined,
                timestamp: Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
            },
            user_session::Model {
                id: 12,
                user_id: 22,
                session_id: 1,
                proxy: Some("33".to_string()),
                join_left: JoinLeft::Joined,
                timestamp: Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
            },
        ];

        let err = select_voter_instance(&voters, None).expect_err("id should be required");
        assert_eq!(
            err,
            "Multiple vote instances available; voter_instance_id is required"
        );
    }

    #[test]
    fn select_voter_instance_picks_requested_instance() {
        let voters = vec![
            user_session::Model {
                id: 11,
                user_id: 22,
                session_id: 1,
                proxy: None,
                join_left: JoinLeft::Joined,
                timestamp: Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
            },
            user_session::Model {
                id: 12,
                user_id: 22,
                session_id: 1,
                proxy: Some("33".to_string()),
                join_left: JoinLeft::Joined,
                timestamp: Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
            },
        ];

        let selected = select_voter_instance(&voters, Some(12)).expect("instance should exist");
        assert_eq!(selected.id, 12);
        assert_eq!(selected.proxy, Some("33".to_string()));
    }
}

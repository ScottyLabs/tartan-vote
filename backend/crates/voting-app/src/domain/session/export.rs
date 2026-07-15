use crate::AppState;
use crate::core::auth::middleware::SyncedUser;
use crate::core::error::AppError;
use crate::domain::attendance::handlers::get_attendance;
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use entity::enums::JoinLeft;
use entity::event;
use entity::session::{self, Entity as Session};
use genpdf::elements::FrameCellDecorator;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter,
    QueryOrder, Statement,
};
use std::collections::HashMap;
use voting_app_store::Store;

#[cfg(test)]
use sea_orm::Database;

#[derive(Debug, Clone, Copy)]
enum ExportKind {
    Attendance,
    Vote,
}

#[derive(Debug, Clone, Copy)]
enum ExportFormat {
    Pdf,
    Csv,
}

impl ExportKind {
    fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "attendance" => Some(Self::Attendance),
            "vote" | "votes" => Some(Self::Vote),
            _ => None,
        }
    }
}

impl ExportFormat {
    fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "pdf" => Some(Self::Pdf),
            "csv" => Some(Self::Csv),
            _ => None,
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Csv => "csv",
        }
    }

    fn content_type(self) -> &'static str {
        match self {
            Self::Pdf => "application/pdf",
            Self::Csv => "text/csv; charset=utf-8",
        }
    }
}

#[cfg(test)]
async fn connect() -> (DatabaseConnection, Store) {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&url)
        .await
        .expect("Failed to connect to database");
    let store = Store::new(db.clone());
    (db, store)
}

#[derive(Debug, FromQueryResult)]
struct VoteCount {
    option: String,
    count: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct SessionEventExportItem {
    pub event_id: i32,
    pub event_name: String,
    pub event_type: String,
    pub status: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub total_votes: u32,
    pub option_counts: Vec<SessionEventOptionCount>,
}

#[derive(Debug, serde::Serialize)]
pub struct SessionEventOptionCount {
    pub option: String,
    pub count: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct SessionEventsExportResponse {
    pub session_code: String,
    pub total_events: usize,
    pub events: Vec<SessionEventExportItem>,
}

#[derive(Debug, Clone)]
struct AttendanceRow {
    user: entity::user::Model,
    proxy_for: Vec<String>,
}

async fn get_vote_counts(
    session_code: &str,
    db: &DatabaseConnection,
) -> Option<Vec<(String, i64)>> {
    let session = Session::find()
        .filter(session::Column::JoinCode.eq(session_code))
        .one(db)
        .await
        .ok()??;

    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        SELECT vote.data->'vote_response'->>0 AS option, COUNT(*) AS count
        FROM vote
        JOIN event ON vote.event_id = event.id
        WHERE event.session_id = $1
        GROUP BY vote.data->'vote_response'->>0
        "#,
        [session.id.into()],
    );

    let counts: Vec<VoteCount> = VoteCount::find_by_statement(stmt).all(db).await.ok()?;

    Some(counts.into_iter().map(|r| (r.option, r.count)).collect())
}

fn load_font_family() -> genpdf::fonts::FontFamily<genpdf::fonts::FontData> {
    let font_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fonts");
    genpdf::fonts::FontFamily {
        regular: genpdf::fonts::FontData::new(
            std::fs::read(format!("{}/liberation-sans.regular.ttf", font_dir))
                .expect("Failed to read regular font"),
            None,
        )
        .expect("Failed to load regular font"),
        bold: genpdf::fonts::FontData::new(
            std::fs::read(format!("{}/liberation-sans.bold.ttf", font_dir))
                .expect("Failed to read bold font"),
            None,
        )
        .expect("Failed to load bold font"),
        italic: genpdf::fonts::FontData::new(
            std::fs::read(format!("{}/liberation-sans.italic.ttf", font_dir))
                .expect("Failed to read italic font"),
            None,
        )
        .expect("Failed to load italic font"),
        bold_italic: genpdf::fonts::FontData::new(
            std::fs::read(format!("{}/liberation-sans.bold-italic.ttf", font_dir))
                .expect("Failed to read bold-italic font"),
            None,
        )
        .expect("Failed to load bold-italic font"),
    }
}

fn build_attendance_csv(rows: &[AttendanceRow]) -> Vec<u8> {
    let mut csv = String::from("User ID,Name,Andrew ID,Is Proxy Holder,Proxy For\n");
    for row in rows {
        let proxy_for = if row.proxy_for.is_empty() {
            String::new()
        } else {
            row.proxy_for.join("; ")
        };

        csv.push_str(&format!(
            "{},{},{},{},{}\n",
            row.user.id,
            row.user.name,
            row.user.andrew_id,
            !row.proxy_for.is_empty(),
            proxy_for
        ));
    }
    csv.into_bytes()
}

fn build_vote_csv(counts: &[(String, i64)]) -> Vec<u8> {
    let total: i64 = counts.iter().map(|(_, c)| c).sum();
    let mut csv = String::from("Option,Count,%\n");
    for (option, count) in counts {
        let pct = if total > 0 { count * 100 / total } else { 0 };
        csv.push_str(&format!("{},{},{}\n", option, count, pct));
    }
    csv.into_bytes()
}

fn build_attendance_pdf(session_code: &str, rows: &[AttendanceRow]) -> Vec<u8> {
    let mut doc = genpdf::Document::new(load_font_family());
    doc.set_title(format!("Session Attendance: {}", session_code));
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(genpdf::elements::Paragraph::new(format!(
        "Session: {}",
        session_code
    )));
    doc.push(genpdf::elements::Paragraph::new(format!(
        "Total Attendees: {}",
        rows.len()
    )));
    doc.push(genpdf::elements::Break::new(1));

    doc.push(genpdf::elements::Paragraph::new("--- Attendance ---"));
    let mut table = genpdf::elements::TableLayout::new(vec![1, 2, 2, 1, 3]);
    table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
    let cell = |text: &str| {
        genpdf::elements::PaddedElement::new(
            genpdf::elements::Paragraph::new(text).aligned(genpdf::Alignment::Center),
            genpdf::Margins::trbl(2, 1, 2, 1),
        )
    };

    let mut header = table.row();
    header.push_element(cell("User ID"));
    header.push_element(cell("Name"));
    header.push_element(cell("Andrew ID"));
    header.push_element(cell("Proxy Holder"));
    header.push_element(cell("Proxy For"));
    header.push().expect("Failed to push header row");

    for attendance_row in rows {
        let mut table_row = table.row();
        table_row.push_element(cell(&attendance_row.user.id.to_string()));
        table_row.push_element(cell(&attendance_row.user.name));
        table_row.push_element(cell(&attendance_row.user.andrew_id));
        table_row.push_element(cell(if attendance_row.proxy_for.is_empty() {
            "No"
        } else {
            "Yes"
        }));
        table_row.push_element(cell(&attendance_row.proxy_for.join(", ")));
        table_row.push().expect("Failed to push row");
    }
    doc.push(table);

    let mut buf = Vec::new();
    doc.render(&mut buf).expect("Failed to render PDF");
    buf
}

fn build_vote_pdf(session_code: &str, counts: &[(String, i64)]) -> Vec<u8> {
    let total: i64 = counts.iter().map(|(_, c)| c).sum();
    let mut doc = genpdf::Document::new(load_font_family());
    doc.set_title(format!("Vote Results: {}", session_code));
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(genpdf::elements::Paragraph::new(format!(
        "Session: {}",
        session_code
    )));
    doc.push(genpdf::elements::Paragraph::new(format!(
        "Total Votes: {}",
        total
    )));
    doc.push(genpdf::elements::Break::new(1));

    doc.push(genpdf::elements::Paragraph::new("--- Results ---"));
    for (option, count) in counts {
        let pct = if total > 0 { count * 100 / total } else { 0 };
        doc.push(genpdf::elements::Paragraph::new(format!(
            "{}: {} ({}%)",
            option, count, pct
        )));
    }
    doc.push(genpdf::elements::Break::new(1));

    let mut table = genpdf::elements::TableLayout::new(vec![3, 1, 1]);
    table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
    let cell = |text: &str| {
        genpdf::elements::PaddedElement::new(
            genpdf::elements::Paragraph::new(text).aligned(genpdf::Alignment::Center),
            genpdf::Margins::trbl(2, 1, 2, 1),
        )
    };

    let mut header = table.row();
    header.push_element(cell("Option"));
    header.push_element(cell("Count"));
    header.push_element(cell("%"));
    header.push().expect("Failed to push header row");

    for (option, count) in counts {
        let pct = if total > 0 { count * 100 / total } else { 0 };
        let mut row = table.row();
        row.push_element(cell(option));
        row.push_element(cell(&count.to_string()));
        row.push_element(cell(&format!("{}%", pct)));
        row.push().expect("Failed to push row");
    }
    doc.push(table);

    let mut buf = Vec::new();
    doc.render(&mut buf).expect("Failed to render PDF");
    buf
}

async fn get_attendance_rows_with_store(store: &Store, session_code: &str) -> Vec<AttendanceRow> {
    let users = get_attendance(store, session_code)
        .await
        .unwrap_or_default();

    let Some(session) = store
        .sessions()
        .find_by_join_code(session_code.to_string())
        .await
        .ok()
        .flatten()
    else {
        return users
            .into_iter()
            .map(|user| AttendanceRow {
                user,
                proxy_for: Vec::new(),
            })
            .collect();
    };

    let user_session_rows = store
        .user_sessions()
        .fetch_by_session_id(session.id)
        .await
        .unwrap_or_default();

    let mut proxy_map: HashMap<i32, Vec<String>> = HashMap::new();
    for session_row in user_session_rows {
        if session_row.join_left != JoinLeft::Joined {
            continue;
        }

        let Some(proxy_for) = session_row.proxy else {
            continue;
        };

        proxy_map
            .entry(session_row.user_id)
            .or_default()
            .push(proxy_for);
    }

    users
        .into_iter()
        .map(|user| AttendanceRow {
            proxy_for: proxy_map.remove(&user.id).unwrap_or_default(),
            user,
        })
        .collect()
}

pub async fn ret_attendance_pdf_with_store(store: &Store, session_code: &str) -> Vec<u8> {
    let rows = get_attendance_rows_with_store(store, session_code).await;
    build_attendance_pdf(session_code, &rows)
}

pub async fn ret_attendance_csv_with_store(store: &Store, session_code: &str) -> Vec<u8> {
    let rows = get_attendance_rows_with_store(store, session_code).await;
    build_attendance_csv(&rows)
}

pub async fn ret_vote_pdf_with_db(db: &DatabaseConnection, session_code: &str) -> Vec<u8> {
    let counts = get_vote_counts(session_code, db).await.unwrap_or_default();
    build_vote_pdf(session_code, &counts)
}

pub async fn ret_vote_csv_with_db(db: &DatabaseConnection, session_code: &str) -> Vec<u8> {
    let counts = get_vote_counts(session_code, db).await.unwrap_or_default();
    build_vote_csv(&counts)
}

pub async fn export_session_data(
    _user: SyncedUser,
    State(state): State<AppState>,
    Path((session_code, kind, format)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let kind = ExportKind::parse(&kind)
        .ok_or_else(|| AppError::bad_request("Invalid export kind. Use 'attendance' or 'vote'."))?;

    let format = ExportFormat::parse(&format)
        .ok_or_else(|| AppError::bad_request("Invalid export format. Use 'pdf' or 'csv'."))?;

    let bytes = match (kind, format) {
        (ExportKind::Attendance, ExportFormat::Pdf) => {
            ret_attendance_pdf_with_store(&state.store, &session_code).await
        }
        (ExportKind::Attendance, ExportFormat::Csv) => {
            ret_attendance_csv_with_store(&state.store, &session_code).await
        }
        (ExportKind::Vote, ExportFormat::Pdf) => {
            ret_vote_pdf_with_db(&state.db, &session_code).await
        }
        (ExportKind::Vote, ExportFormat::Csv) => {
            ret_vote_csv_with_db(&state.db, &session_code).await
        }
    };

    let filename = format!(
        "{}_{}.{}",
        session_code,
        match kind {
            ExportKind::Attendance => "attendance",
            ExportKind::Vote => "vote",
        },
        format.extension()
    );

    let content_disposition =
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .map_err(|_| AppError::internal("Failed to build response headers"))?;

    Ok((
        StatusCode::OK,
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static(format.content_type()),
            ),
            (header::CONTENT_DISPOSITION, content_disposition),
        ],
        bytes,
    )
        .into_response())
}

pub async fn export_session_events_json(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let session = Session::find()
        .filter(session::Column::JoinCode.eq(session_code.clone()))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::not_found("Session not found"))?;

    if session.created_by_user_id != user.0.id {
        return Err(AppError::forbidden(
            "Only the session host may export events",
        ));
    }

    let events = entity::prelude::Event::find()
        .filter(event::Column::SessionId.eq(session.id))
        .order_by_asc(event::Column::StartTime)
        .all(&state.db)
        .await?;

    let mut event_exports = Vec::with_capacity(events.len());

    for session_event in events {
        let vote_rows = entity::prelude::Vote::find()
            .filter(entity::vote::Column::EventId.eq(session_event.id))
            .all(&state.db)
            .await?;

        let option_labels = session_event
            .data
            .get("vote_options")
            .and_then(|value| value.as_array())
            .map(|options| {
                options
                    .iter()
                    .filter_map(|option| option.as_str().map(ToOwned::to_owned))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mut counts: std::collections::HashMap<String, u32> = option_labels
            .iter()
            .map(|option| (option.clone(), 0u32))
            .collect();

        let mut total_votes = 0u32;

        for vote_row in vote_rows {
            let Some(selected_option) = vote_row
                .data
                .get("vote_response")
                .and_then(|value| value.as_array())
                .and_then(|responses| responses.first())
                .and_then(|response| response.as_str())
            else {
                continue;
            };

            if let Some(count) = counts.get_mut(selected_option) {
                *count += 1;
            } else {
                counts.insert(selected_option.to_string(), 1);
            }

            total_votes += 1;
        }

        let mut option_counts = counts
            .into_iter()
            .map(|(option, count)| SessionEventOptionCount { option, count })
            .collect::<Vec<_>>();
        option_counts.sort_by(|a, b| a.option.cmp(&b.option));

        event_exports.push(SessionEventExportItem {
            event_id: session_event.id,
            event_name: session_event.name,
            event_type: format!("{:?}", session_event.event_type),
            status: format!("{:?}", session_event.status),
            start_time: session_event.start_time.to_rfc3339(),
            end_time: session_event.end_time.map(|value| value.to_rfc3339()),
            total_votes,
            option_counts,
        });
    }

    Ok((
        StatusCode::OK,
        Json(SessionEventsExportResponse {
            session_code,
            total_events: event_exports.len(),
            events: event_exports,
        }),
    )
        .into_response())
}

#[cfg(test)]
pub async fn ret_attendance_pdf(session_code: &str) -> Vec<u8> {
    let (_, store) = connect().await;
    ret_attendance_pdf_with_store(&store, session_code).await
}

#[cfg(test)]
pub async fn ret_attendance_csv(session_code: &str) -> Vec<u8> {
    let (_, store) = connect().await;
    ret_attendance_csv_with_store(&store, session_code).await
}

#[cfg(test)]
pub async fn ret_vote_pdf(session_code: &str) -> Vec<u8> {
    let (db, _) = connect().await;
    ret_vote_pdf_with_db(&db, session_code).await
}

#[cfg(test)]
pub async fn ret_vote_csv(session_code: &str) -> Vec<u8> {
    let (db, _) = connect().await;
    ret_vote_csv_with_db(&db, session_code).await
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use chrono::Utc;
    use entity::enums::{EventType, JoinLeft, SessionStatus, StatusOption};
    use entity::{event, session, user, user_session, vote};
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, EntityTrait};
    use serde_json::json;

    async fn test_db() -> DatabaseConnection {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Database::connect(&url).await.expect("Failed to connect")
    }

    // Cleans up any leftover data from a prior failed run for the given session code.
    async fn cleanup(db: &DatabaseConnection, session_code: &str) {
        use entity::session::Entity as Session;
        use sea_orm::{ColumnTrait, QueryFilter};
        if let Ok(Some(s)) = Session::find()
            .filter(session::Column::JoinCode.eq(session_code))
            .one(db)
            .await
        {
            // Deleting the creator user cascades: session → events → votes, user_sessions
            let _ = entity::user::Entity::delete_by_id(s.created_by_user_id)
                .exec(db)
                .await;
        }
    }

    async fn insert_user(db: &DatabaseConnection, name: &str, andrew_id: &str) -> user::Model {
        user::ActiveModel {
            name: Set(name.to_string()),
            andrew_id: Set(andrew_id.to_string()),
            oidc_subject: Set(format!("sub_{}", andrew_id)),
            created_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
        .insert(db)
        .await
        .expect("Failed to insert user")
    }

    async fn insert_session(db: &DatabaseConnection, code: &str, user_id: i32) -> session::Model {
        session::ActiveModel {
            join_code: Set(code.to_string()),
            status: Set(SessionStatus::Open),
            created_by_user_id: Set(user_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .expect("Failed to insert session")
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL and font files"]
    async fn test_attendance_pdf_with_db() {
        let db = test_db().await;
        let code = "ATST01";
        cleanup(&db, code).await;

        let user = insert_user(&db, "Test Alice", "talice").await;
        let sess = insert_session(&db, code, user.id).await;

        user_session::ActiveModel {
            user_id: Set(user.id),
            session_id: Set(sess.id),
            join_left: Set(JoinLeft::Joined),
            timestamp: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .expect("Failed to insert user_session");

        let bytes = ret_attendance_pdf(code).await;
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");

        let path = std::env::current_dir().unwrap().join("test_attendance.pdf");
        std::fs::write(&path, &bytes).expect("Failed to write PDF");
        println!("Saved: {}", path.display());

        entity::user::Entity::delete_by_id(user.id)
            .exec(&db)
            .await
            .expect("Failed to cleanup");
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL"]
    async fn test_attendance_csv_with_db() {
        let db = test_db().await;
        let code = "ATST02";
        cleanup(&db, code).await;

        let user = insert_user(&db, "Test Bob", "tbob").await;
        let sess = insert_session(&db, code, user.id).await;

        user_session::ActiveModel {
            user_id: Set(user.id),
            session_id: Set(sess.id),
            join_left: Set(JoinLeft::Joined),
            timestamp: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .expect("Failed to insert user_session");

        user_session::ActiveModel {
            user_id: Set(user.id),
            session_id: Set(sess.id),
            join_left: Set(JoinLeft::Joined),
            proxy: Set(Some("Proxy Target".to_string())),
            timestamp: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .expect("Failed to insert proxy user_session");

        let bytes = ret_attendance_csv(code).await;
        let csv = String::from_utf8(bytes).unwrap();
        assert!(csv.starts_with("User ID,Name,Andrew ID,Is Proxy Holder,Proxy For\n"));
        assert!(csv.contains(&user.id.to_string()));
        assert!(csv.contains("tbob"));
        assert!(csv.contains("true,Proxy Target"));

        entity::user::Entity::delete_by_id(user.id)
            .exec(&db)
            .await
            .expect("Failed to cleanup");
    }

    async fn insert_user_session(
        db: &DatabaseConnection,
        user_id: i32,
        session_id: i32,
        join_left: JoinLeft,
    ) {
        user_session::ActiveModel {
            user_id: Set(user_id),
            session_id: Set(session_id),
            join_left: Set(join_left),
            timestamp: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
        .insert(db)
        .await
        .expect("Failed to insert user_session");
    }

    async fn insert_event(db: &DatabaseConnection, sess_id: i32, creator_id: i32) -> event::Model {
        event::ActiveModel {
            event_type: Set(EventType::Motion),
            name: Set("Test Motion".to_string()),
            status: Set(StatusOption::Active),
            start_time: Set(Utc::now().fixed_offset()),
            end_time: Set(None),
            data: Set(json!({
                "vote_options": ["pass", "reject", "abstain"],
                "threshold": 0.5,
                "proxy": false,
                "visibility": { "participants": "live" }
            })),
            session_id: Set(sess_id),
            created_by_user_id: Set(creator_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .expect("Failed to insert event")
    }

    async fn insert_vote_for(
        db: &DatabaseConnection,
        session_id: i32,
        event_id: i32,
        user_id: i32,
        response: &str,
    ) {
        let user_session_row = user_session::ActiveModel {
            user_id: Set(user_id),
            session_id: Set(session_id),
            proxy: Set(None),
            join_left: Set(JoinLeft::Joined),
            timestamp: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
        .insert(db)
        .await
        .expect("Failed to insert user_session");

        vote::ActiveModel {
            event_id: Set(event_id),
            user_session_id: Set(user_session_row.id),
            cast_time: Set(Utc::now().fixed_offset()),
            data: Set(json!({
                "vote_type": "motion",
                "proxy": false,
                "vote_response": [response]
            })),
            ..Default::default()
        }
        .insert(db)
        .await
        .expect("Failed to insert vote");
    }

    // --- vote PDF / CSV ---

    #[tokio::test]
    #[ignore = "requires DATABASE_URL and font files"]
    async fn test_vote_pdf_single() {
        let db = test_db().await;
        let code = "VTST01";
        cleanup(&db, code).await;

        let user = insert_user(&db, "Test Voter", "tvoter").await;
        let sess = insert_session(&db, code, user.id).await;
        let evt = insert_event(&db, sess.id, user.id).await;
        insert_vote_for(&db, sess.id, evt.id, user.id, "pass").await;

        let bytes = ret_vote_pdf(code).await;
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");

        let path = std::env::current_dir()
            .unwrap()
            .join("test_vote_single.pdf");
        std::fs::write(&path, &bytes).expect("Failed to write PDF");
        println!("Saved: {}", path.display());

        entity::user::Entity::delete_by_id(user.id)
            .exec(&db)
            .await
            .expect("cleanup");
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL"]
    async fn test_vote_csv_single() {
        let db = test_db().await;
        let code = "VTST02";
        cleanup(&db, code).await;

        let user = insert_user(&db, "Test Voter2", "tvoter2").await;
        let sess = insert_session(&db, code, user.id).await;
        let evt = insert_event(&db, sess.id, user.id).await;
        insert_vote_for(&db, sess.id, evt.id, user.id, "pass").await;

        let bytes = ret_vote_csv(code).await;
        let csv = String::from_utf8(bytes).unwrap();
        assert!(csv.starts_with("Option,Count,%\n"));
        assert!(csv.contains("pass,1,100"));

        entity::user::Entity::delete_by_id(user.id)
            .exec(&db)
            .await
            .expect("cleanup");
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL and font files"]
    async fn test_vote_pdf_split_votes() {
        // 10 voters: 5 pass, 3 reject, 2 abstain
        let db = test_db().await;
        let code = "VTST03";
        cleanup(&db, code).await;

        let host = insert_user(&db, "Host", "host03").await;
        let sess = insert_session(&db, code, host.id).await;
        let evt = insert_event(&db, sess.id, host.id).await;

        let responses = [("pass", 5), ("reject", 3), ("abstain", 2)];
        let mut all_users = vec![host.id];
        let mut i = 0u32;
        for (response, count) in responses {
            for _ in 0..count {
                i += 1;
                let u = insert_user(&db, &format!("Voter{i}"), &format!("v{i}03")).await;
                insert_vote_for(&db, sess.id, evt.id, u.id, response).await;
                all_users.push(u.id);
            }
        }

        let bytes = ret_vote_pdf(code).await;
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");

        let path = std::env::current_dir().unwrap().join("test_vote_split.pdf");
        std::fs::write(&path, &bytes).expect("Failed to write PDF");
        println!("Saved: {}", path.display());

        for uid in all_users {
            let _ = entity::user::Entity::delete_by_id(uid).exec(&db).await;
        }
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL"]
    async fn test_vote_csv_split_votes() {
        // 6 voters: 3 pass (50%), 2 reject (33%), 1 abstain (16%)
        let db = test_db().await;
        let code = "VTST04";
        cleanup(&db, code).await;

        let host = insert_user(&db, "Host4", "host04").await;
        let sess = insert_session(&db, code, host.id).await;
        let evt = insert_event(&db, sess.id, host.id).await;

        let responses = [("pass", 3), ("reject", 2), ("abstain", 1)];
        let mut all_users = vec![host.id];
        let mut i = 0u32;
        for (response, count) in responses {
            for _ in 0..count {
                i += 1;
                let u = insert_user(&db, &format!("Voter{i}"), &format!("v{i}04")).await;
                insert_vote_for(&db, sess.id, evt.id, u.id, response).await;
                all_users.push(u.id);
            }
        }

        let bytes = ret_vote_csv(code).await;
        let csv = String::from_utf8(bytes).unwrap();
        assert!(csv.starts_with("Option,Count,%\n"));
        assert!(csv.contains("pass,3,50"));
        assert!(csv.contains("reject,2,33"));
        assert!(csv.contains("abstain,1,16"));

        for uid in all_users {
            let _ = entity::user::Entity::delete_by_id(uid).exec(&db).await;
        }
    }

    // --- attendance PDF / CSV ---

    #[tokio::test]
    #[ignore = "requires DATABASE_URL and font files"]
    async fn test_attendance_pdf_single() {
        let db = test_db().await;
        let code = "ATST01";
        cleanup(&db, code).await;

        let user = insert_user(&db, "Test Alice", "talice").await;
        let sess = insert_session(&db, code, user.id).await;
        insert_user_session(&db, user.id, sess.id, JoinLeft::Joined).await;

        let bytes = ret_attendance_pdf(code).await;
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");

        let path = std::env::current_dir()
            .unwrap()
            .join("test_attendance_single.pdf");
        std::fs::write(&path, &bytes).expect("Failed to write PDF");
        println!("Saved: {}", path.display());

        entity::user::Entity::delete_by_id(user.id)
            .exec(&db)
            .await
            .expect("cleanup");
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL"]
    async fn test_attendance_csv_single() {
        let db = test_db().await;
        let code = "ATST05";
        cleanup(&db, code).await;

        let user = insert_user(&db, "Test Bob", "tbob").await;
        let sess = insert_session(&db, code, user.id).await;
        insert_user_session(&db, user.id, sess.id, JoinLeft::Joined).await;

        let bytes = ret_attendance_csv(code).await;
        let csv = String::from_utf8(bytes).unwrap();
        assert!(csv.starts_with("User ID,Name,Andrew ID,Is Proxy Holder,Proxy For\n"));
        assert!(csv.contains("tbob"));

        entity::user::Entity::delete_by_id(user.id)
            .exec(&db)
            .await
            .expect("cleanup");
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL and font files"]
    async fn test_attendance_pdf_many_users() {
        // 10 users all joined
        let db = test_db().await;
        let code = "ATST03";
        cleanup(&db, code).await;

        let host = insert_user(&db, "Host", "host03a").await;
        let sess = insert_session(&db, code, host.id).await;
        let mut all_users = vec![host.id];

        for i in 1..=10u32 {
            let u = insert_user(&db, &format!("Attendee {i}"), &format!("att{i}03")).await;
            insert_user_session(&db, u.id, sess.id, JoinLeft::Joined).await;
            all_users.push(u.id);
        }

        let bytes = ret_attendance_pdf(code).await;
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");

        let path = std::env::current_dir()
            .unwrap()
            .join("test_attendance_many.pdf");
        std::fs::write(&path, &bytes).expect("Failed to write PDF");
        println!("Saved: {}", path.display());

        for uid in all_users {
            let _ = entity::user::Entity::delete_by_id(uid).exec(&db).await;
        }
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL"]
    async fn test_attendance_csv_mixed_join_left() {
        // 5 joined, 3 left — CSV should include all 8
        let db = test_db().await;
        let code = "ATST04";
        cleanup(&db, code).await;

        let host = insert_user(&db, "Host4", "host04a").await;
        let sess = insert_session(&db, code, host.id).await;
        let mut all_users = vec![host.id];

        for i in 1..=5u32 {
            let u = insert_user(&db, &format!("Joiner{i}"), &format!("j{i}04")).await;
            insert_user_session(&db, u.id, sess.id, JoinLeft::Joined).await;
            all_users.push(u.id);
        }
        for i in 1..=3u32 {
            let u = insert_user(&db, &format!("Leaver{i}"), &format!("l{i}04")).await;
            insert_user_session(&db, u.id, sess.id, JoinLeft::Left).await;
            all_users.push(u.id);
        }

        let bytes = ret_attendance_csv(code).await;
        let csv = String::from_utf8(bytes).unwrap();
        let lines: Vec<&str> = csv.lines().collect();
        // header + 8 data rows
        assert_eq!(lines.len(), 9);
        assert!(csv.contains("j104"));
        assert!(csv.contains("l104"));

        for uid in all_users {
            let _ = entity::user::Entity::delete_by_id(uid).exec(&db).await;
        }
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL and font files"]
    async fn test_vote_pdf_large() {
        // 20 voters: 12 pass, 5 reject, 3 abstain
        let db = test_db().await;
        let code = "VTST05";
        cleanup(&db, code).await;

        let host = insert_user(&db, "Host5", "host05").await;
        let sess = insert_session(&db, code, host.id).await;
        let evt = insert_event(&db, sess.id, host.id).await;

        let responses = [("pass", 12), ("reject", 5), ("abstain", 3)];
        let mut all_users = vec![host.id];
        let mut i = 0u32;
        for (response, count) in responses {
            for _ in 0..count {
                i += 1;
                let u = insert_user(&db, &format!("Voter{i}"), &format!("v{i}05")).await;
                insert_vote_for(&db, sess.id, evt.id, u.id, response).await;
                all_users.push(u.id);
            }
        }

        let bytes = ret_vote_pdf(code).await;
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");

        let path = std::env::current_dir().unwrap().join("test_vote_large.pdf");
        std::fs::write(&path, &bytes).expect("Failed to write PDF");
        println!("Saved: {}", path.display());

        for uid in all_users {
            let _ = entity::user::Entity::delete_by_id(uid).exec(&db).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use entity::user;

    fn mock_user(id: i32, name: &str, andrew_id: &str) -> user::Model {
        user::Model {
            id,
            name: name.to_string(),
            andrew_id: andrew_id.to_string(),
            oidc_subject: format!("sub_{}", id),
            created_at: Utc::now().fixed_offset(),
        }
    }

    fn mock_attendance_row(
        id: i32,
        name: &str,
        andrew_id: &str,
        proxy_for: Vec<&str>,
    ) -> AttendanceRow {
        AttendanceRow {
            user: mock_user(id, name, andrew_id),
            proxy_for: proxy_for.into_iter().map(str::to_string).collect(),
        }
    }

    // --- attendance CSV ---

    #[test]
    fn test_attendance_csv_header_only() {
        let bytes = build_attendance_csv(&[]);
        let csv = String::from_utf8(bytes).unwrap();
        assert_eq!(csv, "User ID,Name,Andrew ID,Is Proxy Holder,Proxy For\n");
    }

    #[test]
    fn test_attendance_csv_single_user() {
        let rows = vec![mock_attendance_row(1, "Alice", "alice1", vec![])];
        let bytes = build_attendance_csv(&rows);
        let csv = String::from_utf8(bytes).unwrap();
        assert_eq!(
            csv,
            "User ID,Name,Andrew ID,Is Proxy Holder,Proxy For\n1,Alice,alice1,false,\n"
        );
    }

    #[test]
    fn test_attendance_csv_multiple_users() {
        let rows = vec![
            mock_attendance_row(1, "Alice", "alice1", vec![]),
            mock_attendance_row(2, "Bob", "bob2", vec!["Senator Zed"]),
            mock_attendance_row(3, "Carol", "carol3", vec![]),
        ];
        let bytes = build_attendance_csv(&rows);
        let csv = String::from_utf8(bytes).unwrap();
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines[0], "User ID,Name,Andrew ID,Is Proxy Holder,Proxy For");
        assert_eq!(lines[1], "1,Alice,alice1,false,");
        assert_eq!(lines[2], "2,Bob,bob2,true,Senator Zed");
        assert_eq!(lines[3], "3,Carol,carol3,false,");
        assert_eq!(lines.len(), 4);
    }

    // --- vote CSV ---

    #[test]
    fn test_vote_csv_header_only() {
        let bytes = build_vote_csv(&[]);
        let csv = String::from_utf8(bytes).unwrap();
        assert_eq!(csv, "Option,Count,%\n");
    }

    #[test]
    fn test_vote_csv_single_option() {
        let counts = vec![("pass".to_string(), 10)];
        let bytes = build_vote_csv(&counts);
        let csv = String::from_utf8(bytes).unwrap();
        assert_eq!(csv, "Option,Count,%\npass,10,100\n");
    }

    #[test]
    fn test_vote_csv_multiple_options() {
        let counts = vec![
            ("pass".to_string(), 6),
            ("reject".to_string(), 3),
            ("abstain".to_string(), 1),
        ];
        let bytes = build_vote_csv(&counts);
        let csv = String::from_utf8(bytes).unwrap();
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines[0], "Option,Count,%");
        assert_eq!(lines[1], "pass,6,60");
        assert_eq!(lines[2], "reject,3,30");
        assert_eq!(lines[3], "abstain,1,10");
    }

    #[test]
    fn test_vote_csv_zero_total() {
        let counts = vec![("pass".to_string(), 0)];
        let bytes = build_vote_csv(&counts);
        let csv = String::from_utf8(bytes).unwrap();
        assert!(csv.contains("pass,0,0"));
    }

    // --- attendance PDF (requires fonts) ---

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_attendance_pdf_returns_bytes() {
        let rows = vec![
            mock_attendance_row(1, "Alice", "alice1", vec![]),
            mock_attendance_row(2, "Bob", "bob2", vec!["Senator A"]),
        ];
        let bytes = build_attendance_pdf("ABC123", &rows);
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");
    }

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_attendance_pdf_empty() {
        let bytes = build_attendance_pdf("ABC123", &[]);
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");
    }

    #[test]
    #[ignore = "preview only — writes pdf to /tmp and does not clean up"]
    fn test_attendance_pdf_preview() {
        let rows = vec![
            mock_attendance_row(1, "Alice", "alice1", vec![]),
            mock_attendance_row(2, "Bob", "bob2", vec!["Senator A"]),
            mock_attendance_row(3, "Carol", "carol3", vec!["Senator B", "Senator C"]),
        ];
        let bytes = build_attendance_pdf("ABC123", &rows);
        std::fs::write("/tmp/test_attendance.pdf", &bytes).expect("Failed to write PDF");
    }

    // --- vote PDF (requires fonts) ---

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_vote_pdf_returns_bytes() {
        let counts = vec![
            ("pass".to_string(), 6),
            ("reject".to_string(), 3),
            ("abstain".to_string(), 1),
        ];
        let bytes = build_vote_pdf("ABC123", &counts);
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");
    }

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_vote_pdf_empty() {
        let bytes = build_vote_pdf("ABC123", &[]);
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");
    }

    #[test]
    #[ignore = "preview only — writes pdf to /tmp and does not clean up"]
    fn test_vote_pdf_preview() {
        let counts = vec![
            ("pass".to_string(), 6),
            ("reject".to_string(), 3),
            ("abstain".to_string(), 1),
        ];
        let bytes = build_vote_pdf("ABC123", &counts);
        std::fs::write("/tmp/test_vote_results.pdf", &bytes).expect("Failed to write PDF");
    }
}

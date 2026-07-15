use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Forbidden(String),
    BadRequest(String),
    Conflict(String),
    Internal(String),
    WithStatus { status: StatusCode, message: String },
}

impl AppError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::Forbidden(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    pub fn with_status(status: StatusCode, msg: impl Into<String>) -> Self {
        Self::WithStatus {
            status,
            message: msg.into(),
        }
    }

    fn status_and_message(self) -> (StatusCode, String) {
        match self {
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Self::Forbidden(message) => (StatusCode::FORBIDDEN, message),
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Conflict(message) => (StatusCode::CONFLICT, message),
            Self::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            Self::WithStatus { status, message } => (status, message),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = self.status_and_message();
        (status, Json(json!({"error": message}))).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(_err: DbErr) -> Self {
        AppError::internal("Database error")
    }
}

impl From<(StatusCode, String)> for AppError {
    fn from((status, message): (StatusCode, String)) -> Self {
        AppError::with_status(status, message)
    }
}

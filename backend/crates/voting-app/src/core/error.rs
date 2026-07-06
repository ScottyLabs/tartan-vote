use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde_json::json;
use sea_orm::DbErr;


pub struct AppError{
    status: StatusCode,
    message: String,
}

impl AppError {
    //404, 403, 400, 409, 500, 401, 502, 503 
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self { status: StatusCode::NOT_FOUND, message: msg.into()}
    }
    pub fn forbidden(msg: impl Into<String>) -> Self{
        Self {status: StatusCode::FORBIDDEN, message: msg.into()}
    }
    pub fn bad_request(msg: impl Into<String>) -> Self{
        Self {status: StatusCode::BAD_REQUEST, message: msg.into()}
    }
    pub fn conflict(msg: impl Into<String>) -> Self{
        Self {status: StatusCode::CONFLICT, message: msg.into()}
    }
    pub fn internal(msg: impl Into<String>) -> Self{
        Self {status: StatusCode::INTERNAL_SERVER_ERROR, message: msg.into()}
    }
    pub fn unauthorized(msg: impl Into<String>) -> Self{
        Self {status: StatusCode::UNAUTHORIZED, message: msg.into()}
    }
    pub fn bad_gateway(msg: impl Into<String>) -> Self{
        Self {status: StatusCode::BAD_GATEWAY, message: msg.into()}
    }
    pub fn service_unavailable(msg: impl Into<String>) -> Self{
        Self {status: StatusCode::SERVICE_UNAVAILABLE, message: msg.into()}
    }
    //generic constructor
    pub fn with_status(status: StatusCode, msg: impl Into<String>) -> Self{
        Self {status, message: msg.into()}
    }
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response{
        (self.status, Json(json!({"error": self.message}))).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(err:DbErr) -> Self {
        AppError::internal(err.to_string())
    }
}
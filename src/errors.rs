use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Lock error: failed to acquire lock")]
    LockError,

    #[error("Storage error: {0}")]
    StorageError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::LockError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to acquire lock".to_string(),
            ),
            AppError::StorageError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Storage error: {}", e),
            ),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

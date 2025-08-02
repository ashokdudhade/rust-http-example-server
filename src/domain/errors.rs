use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User with id {0} not found")]
    UserNotFound(Uuid),

    #[error("User with email {0} already exists")]
    UserAlreadyExists(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match &self {
            AppError::UserNotFound(_) => (StatusCode::NOT_FOUND, self.to_string(), "USER_NOT_FOUND"),
            AppError::UserAlreadyExists(_) => (StatusCode::CONFLICT, self.to_string(), "USER_ALREADY_EXISTS"),
            AppError::InvalidInput(_) => (StatusCode::BAD_REQUEST, self.to_string(), "INVALID_INPUT"),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database operation failed".to_string(),
                "DATABASE_ERROR",
            ),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error occurred".to_string(),
                "INTERNAL_ERROR",
            ),
            AppError::Config(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Configuration error".to_string(),
                "CONFIG_ERROR",
            ),
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": error_message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }));

        (status, body).into_response()
    }
}

// Convenience type alias
pub type AppResult<T> = Result<T, AppError>;
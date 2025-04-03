use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::auth::AuthError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Auth(#[from] AuthError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Internal error: {0}")]
    MalformedData(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            AppError::Auth(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::MalformedData(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error: Malformed data.".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "code": status.as_u16()
        }));

        (status, body).into_response()
    }
}

use poem::error::ResponseError;
use poem::http::StatusCode;
use poem_openapi::{
    ApiResponse,
    registry::{MetaResponses, Registry},
};

use crate::{auth::AuthError, user::UserError};

pub type AppResult<T> = std::result::Result<T, AppError>;
pub type InfraResult<T> = std::result::Result<T, InfraError>;

/// Failures in the infrastructure layer, which belong to no business domain.
#[derive(Debug, thiserror::Error, Clone)]
pub enum InfraError {
    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Postgres error: {0}")]
    Postgres(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Network error: {0}")]
    Network(String),
}

impl ResponseError for InfraError {
    fn status(&self) -> StatusCode {
        match self {
            InfraError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InfraError::Postgres(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InfraError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InfraError::Network(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

/// Aggregates every domain's error type. Add one transparent variant per module
/// so handlers can return `AppResult<T>` and use `?` on any domain error.
#[derive(Debug, thiserror::Error, Clone)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Infra(#[from] InfraError),
}

impl ResponseError for AppError {
    fn status(&self) -> StatusCode {
        match self {
            AppError::Auth(err) => err.status(),
            AppError::User(err) => err.status(),
            AppError::Infra(err) => err.status(),
        }
    }
}

// Prevents OpenAPI docs from including every error possibility in the response codes.
impl ApiResponse for AppError {
    fn meta() -> MetaResponses {
        MetaResponses {
            responses: Vec::with_capacity(0),
        }
    }

    fn register(_: &mut Registry) {}
}

impl From<sqlx::Error> for InfraError {
    fn from(err: sqlx::Error) -> Self {
        InfraError::Postgres(err.to_string())
    }
}

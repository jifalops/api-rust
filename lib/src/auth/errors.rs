use poem::error::ResponseError;
use poem::http::StatusCode;

#[derive(Debug, thiserror::Error, Clone)]
pub enum AuthError {
    #[error("Invalid credential")]
    InvalidCredential,

    #[error("Invalid user token: {0}")]
    InvalidToken(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Invalid sign-up data: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl ResponseError for AuthError {
    fn status(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredential => StatusCode::UNAUTHORIZED,
            AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            AuthError::Unauthorized(_) => StatusCode::FORBIDDEN,
            AuthError::Validation(_) => StatusCode::BAD_REQUEST,
            AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

use poem::error::ResponseError;
use poem::http::StatusCode;

#[derive(Debug, thiserror::Error, Clone)]
pub enum UserError {
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Invalid user data: {0}")]
    Invalid(String),
}

impl ResponseError for UserError {
    fn status(&self) -> StatusCode {
        match self {
            UserError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            UserError::UserNotFound(_) => StatusCode::NOT_FOUND,
            UserError::Invalid(_) => StatusCode::BAD_REQUEST,
        }
    }
}

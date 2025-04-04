use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credential")]
    InvalidCredential,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Object)]
pub struct Token {
    pub user_id: String,
    pub token: String,
    pub expires: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate, Object)]
pub struct SignUpData {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4, max = 64))]
    pub password: String,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    #[validate(url)]
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate, Object)]
pub struct SignInData {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4, max = 64))]
    pub password: String,
}

use serde::{Deserialize, Serialize};

use crate::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserIdentifier {
    id: Option<String>,
    email: Option<String>,
}
impl UserIdentifier {
    pub fn new(id: Option<String>, email: Option<String>) -> Result<Self, AppError> {
        if id.is_none() && email.is_none() {
            return Err(AppError::Internal(
                "id or email must be provided".to_owned(),
            ));
        }
        Ok(Self { id, email })
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

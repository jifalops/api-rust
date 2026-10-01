use std::fmt::Display;

use poem_openapi::Object;
use serde::{Deserialize, Serialize};

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
    pub email_verified: bool,
    pub password_hash: String,
    pub name: Option<String>,
    pub photo_url: Option<String>,
}

impl User {
    pub fn identifiers(&self) -> Vec<UserIdentifier> {
        vec![
            UserIdentifier::Id(self.id.clone()),
            UserIdentifier::Email(self.email.clone()),
        ]
    }
}

/// The publicly exposable view of a [`User`] — notably without the password hash.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Object)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub photo_url: Option<String>,
}

impl From<&User> for UserProfile {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.clone(),
            email: user.email.clone(),
            email_verified: user.email_verified,
            name: user.name.clone(),
            photo_url: user.photo_url.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UserIdentifier {
    Id(String),
    Email(String),
}

impl Display for UserIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserIdentifier::Id(id) => id,
            UserIdentifier::Email(email) => email,
        };
        write!(f, "{}", s)
    }
}

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::User;

    /// A valid user fixture. The id is unique per call so repeated use against a
    /// shared database doesn't collide.
    pub fn new_user(email: &str) -> User {
        User {
            id: format!("user_{}", uuid::Uuid::new_v4()),
            email: email.to_string(),
            email_verified: false,
            password_hash: "not-a-real-hash".to_string(),
            name: None,
            photo_url: None,
        }
    }
}

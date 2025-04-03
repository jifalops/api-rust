use std::fmt::Display;

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

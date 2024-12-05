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

pub enum UserIdentifier {
    Id(String),
    Email(String),
}

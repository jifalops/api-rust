use std::{fmt::Display, sync::Arc};

use poem::Request;
use poem_openapi::{NewType, Object, SecurityScheme, auth::Bearer};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use validator::Validate;

use super::AuthService;

/// An encoded and signed token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, NewType)]
pub struct Token(String);

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Claims decoded from a token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenUser {
    #[serde(rename = "sub")]
    pub id: String,
    pub email: String,
    #[serde(rename = "iat", with = "time::serde::timestamp")]
    pub issued_at: OffsetDateTime,
    #[serde(rename = "exp", with = "time::serde::timestamp")]
    pub expires_at: OffsetDateTime,
}

/// Bearer token authentication for OpenAPI. Handlers take this as an argument to
/// require a valid token; the decoded claims come back as `BearerAuth(user)`.
#[derive(SecurityScheme)]
#[oai(ty = "bearer", checker = "check_bearer_token")]
pub struct BearerAuth(pub TokenUser);

async fn check_bearer_token(req: &Request, bearer: Bearer) -> Option<TokenUser> {
    if bearer.token.is_empty() {
        return None;
    }
    let auth = req.data::<Arc<AuthService>>()?;
    auth.verify_token(&Token(bearer.token.to_owned())).ok()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate, Object)]
pub struct SignUpData {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 64))]
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
    #[validate(length(min = 8, max = 64))]
    pub password: String,
}

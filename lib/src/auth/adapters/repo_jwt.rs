use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use time::{Duration, OffsetDateTime};

use crate::{
    AppResult,
    auth::{AuthError, AuthRepo, Token, TokenUser},
    user::User,
};

/// Signs and verifies JWTs with a shared secret.
pub struct AuthRepoJwt {
    secret: String,
    algorithm: Algorithm,
    ttl: Duration,
}

impl AuthRepoJwt {
    pub fn new(secret: &str, algorithm: Algorithm, ttl: Duration) -> Self {
        Self {
            secret: secret.to_string(),
            algorithm,
            ttl,
        }
    }
}

#[async_trait::async_trait]
impl AuthRepo for AuthRepoJwt {
    async fn create_token(&self, user: &User) -> AppResult<Token> {
        let issued_at = OffsetDateTime::now_utc();
        let claims = TokenUser {
            id: user.id.clone(),
            email: user.email.clone(),
            issued_at,
            expires_at: issued_at + self.ttl,
        };
        let encoded = encode(
            &Header::new(self.algorithm),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| AuthError::Internal(format!("Failed to encode token: {e}")))?;
        Ok(Token::from(encoded))
    }

    fn verify(&self, token: &Token) -> AppResult<TokenUser> {
        let decoded: TokenData<TokenUser> = decode(
            token.as_ref(),
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(self.algorithm),
        )
        .map_err(|e| AuthError::InvalidToken(format!("Failed to decode token: {e}")))?;
        Ok(decoded.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::testing::new_user;

    fn repo(ttl: Duration) -> AuthRepoJwt {
        AuthRepoJwt::new("test-secret", Algorithm::HS256, ttl)
    }

    #[tokio::test]
    async fn round_trips_a_token() {
        let repo = repo(Duration::hours(1));
        let user = new_user("round@example.com");
        let token = repo.create_token(&user).await.unwrap();
        let claims = repo.verify(&token).unwrap();
        assert_eq!(claims.id, user.id);
        assert_eq!(claims.email, user.email);
    }

    #[tokio::test]
    async fn rejects_an_expired_token() {
        let repo = repo(Duration::hours(-1));
        let token = repo
            .create_token(&new_user("expired@example.com"))
            .await
            .unwrap();
        assert!(repo.verify(&token).is_err());
    }

    #[tokio::test]
    async fn rejects_a_token_signed_with_another_secret() {
        let token = repo(Duration::hours(1))
            .create_token(&new_user("forged@example.com"))
            .await
            .unwrap();
        let other = AuthRepoJwt::new("different-secret", Algorithm::HS256, Duration::hours(1));
        assert!(other.verify(&token).is_err());
    }
}

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;

use crate::{app::Service, user::User, AppError};

use super::{AuthError, Token};

#[async_trait]
pub trait AuthService: Service {
    fn hash_password(&self, password: String) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .to_string();
        Ok(hashed)
    }
    fn verify_password(&self, password: String, hash: &str) -> Result<(), AppError> {
        let parsed = PasswordHash::new(hash).map_err(|e| AppError::Internal(e.to_string()))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AuthError::InvalidCredential.into())
    }
    async fn generate_token(&self, user: User) -> Result<Token, AppError>;
}

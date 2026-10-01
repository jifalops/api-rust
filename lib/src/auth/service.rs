use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use validator::Validate;

use crate::{
    AppResult,
    user::{NewUser, UserIdentifier, UserService},
};

use super::{AuthError, AuthRepo, SignInData, SignUpData, Token, TokenUser};

pub struct AuthService {
    repo: Box<dyn AuthRepo>,
}

impl AuthService {
    pub fn new(repo: impl AuthRepo) -> Self {
        Self {
            repo: Box::new(repo),
        }
    }

    pub async fn sign_up(&self, data: SignUpData, users: &UserService) -> AppResult<Token> {
        data.validate()
            .map_err(|e| AuthError::Validation(e.to_string()))?;
        let new_user = NewUser {
            email: data.email,
            password_hash: self.hash_password(&data.password)?,
            name: data.name,
            photo_url: data.photo_url,
        };
        let created = users.create_user(new_user).await?;
        self.repo.create_token(&created.model).await
    }

    pub async fn sign_in(&self, data: SignInData, users: &UserService) -> AppResult<Token> {
        data.validate()
            .map_err(|e| AuthError::Validation(e.to_string()))?;
        let user = users
            .get_user(UserIdentifier::Email(data.email))
            .await
            // Don't disclose whether the address is registered.
            .map_err(|_| AuthError::InvalidCredential)?
            .model;
        self.verify_password(&data.password, &user.password_hash)?;
        self.repo.create_token(&user).await
    }

    pub fn verify_token(&self, token: &Token) -> AppResult<TokenUser> {
        self.repo.verify(token)
    }

    fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthError::Internal(e.to_string()))?
            .to_string();
        Ok(hashed)
    }

    fn verify_password(&self, password: &str, hash: &str) -> AppResult<()> {
        let parsed = PasswordHash::new(hash).map_err(|e| AuthError::Internal(e.to_string()))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AuthError::InvalidCredential)?;
        Ok(())
    }
}

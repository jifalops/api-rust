use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;

use crate::{
    app::Service,
    user::{NewUser, User, UserIdentifier},
    App, AppError,
};

use super::{AuthError, SignInData, SignUpData, Token};

#[async_trait]
pub trait AuthService: Service {
    async fn sign_up<A: App>(&self, data: SignUpData, app: &A) -> Result<Token, AppError> {
        let new_user = NewUser {
            email: data.email,
            password_hash: self.hash_password(data.password)?,
            name: data.name,
            photo_url: data.photo_url,
        };
        let user = app.user().create_user(new_user).await?;
        let token = self.generate_token(user).await?;
        Ok(token)
    }

    async fn sign_in<A: App>(&self, data: SignInData, app: &A) -> Result<Token, AppError> {
        let user = app
            .user()
            .get_user(UserIdentifier::Email(data.email))
            .await?;
        self.verify_password(data.password, &user.password_hash)?;
        self.generate_token(user).await
    }

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

use axum::async_trait;
use serde_json::Value;
use short_uuid::short;

use crate::AppError;

use super::{NewUser, User};

pub struct UserRepo<T: UserRepoAdapter> {
    adapter: T,
}

impl<T: UserRepoAdapter> UserRepo<T> {
    pub fn new(adapter: T) -> Self {
        Self { adapter }
    }

    fn generate_id(&self) -> String {
        format!("user_{}", short!())
    }

    pub async fn create_user(&self, data: NewUser) -> Result<User, AppError> {
        let user = User {
            id: self.generate_id(),
            email: data.email,
            email_verified: false,
            password_hash: data.password_hash,
            name: data.name,
            photo_url: data.photo_url,
        };
        self.adapter
            .create_user(serde_json::to_value(&user)?)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<User, AppError> {
        let user = self.adapter.get_user_by_id(id).await?;
        Ok(serde_json::from_value(user)?)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, AppError> {
        let user = self.adapter.get_user_by_email(email).await?;
        Ok(serde_json::from_value(user)?)
    }
}

#[async_trait]
pub trait UserRepoAdapter: Sync + Send + 'static {
    async fn create_user(&self, user: Value) -> Result<(), AppError>;
    async fn get_user_by_id(&self, id: &str) -> Result<Value, AppError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Value, AppError>;
}

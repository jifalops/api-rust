use crate::{app::Repo, AppError};

use super::{User, UserIdentifier};

#[async_trait::async_trait]
pub trait UserRepo: Repo {
    async fn create_user(&self, user: &User) -> Result<(), AppError>;

    async fn get_user(&self, ident: UserIdentifier) -> Result<User, AppError>;

    async fn update_user(&self, user: &User) -> Result<(), AppError>;

    async fn delete_user(&self, ident: UserIdentifier) -> Result<(), AppError>;
}

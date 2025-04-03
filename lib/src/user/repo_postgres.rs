use crate::AppError;

use super::{User, UserIdentifier, UserRepo};

pub struct UserRepoPostgres;

#[async_trait::async_trait]
impl UserRepo for UserRepoPostgres {
    async fn create_user(&self, user: &User) -> Result<(), AppError> {
        todo!();
    }

    async fn get_user(&self, ident: UserIdentifier) -> Result<User, AppError> {
        todo!();
    }

    async fn update_user(&self, user: &User) -> Result<(), AppError> {
        todo!();
    }

    async fn delete_user(&self, ident: UserIdentifier) -> Result<(), AppError> {
        todo!();
    }
}

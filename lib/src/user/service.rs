use crate::AppError;

use super::{
    repo::{UserRepo, UserRepoAdapter},
    NewUser, User, UserIdentifier,
};

pub struct UserService<T: UserRepoAdapter> {
    repo: UserRepo<T>,
}

impl<T: UserRepoAdapter> UserService<T> {
    pub fn new(repo: T) -> Self {
        Self {
            repo: UserRepo::new(repo),
        }
    }

    pub async fn create_user(&self, data: NewUser) -> Result<User, AppError> {
        let user = self.repo.create_user(data).await?;
        Ok(user)
    }

    pub async fn get_user(&self, ident: UserIdentifier) -> Result<User, AppError> {
        if let Some(id) = ident.id() {
            let user = self.repo.get_user_by_id(id).await?;
            return Ok(user);
        }
        if let Some(email) = ident.email() {
            let user = self.repo.get_user_by_email(email).await?;
            return Ok(user);
        }
        Err(AppError::Internal(
            "id or email must be provided".to_owned(),
        ))
    }
}

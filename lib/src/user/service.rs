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
        match ident {
            UserIdentifier::Id(id) => {
                let user = self.repo.get_user_by_id(&id).await?;
                Ok(user)
            }
            UserIdentifier::Email(email) => {
                let user = self.repo.get_user_by_email(&email).await?;
                Ok(user)
            }
        }
    }
}

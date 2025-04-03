use short_uuid::short;

use crate::AppError;

use super::{repo::UserRepo, NewUser, User, UserIdentifier};

pub struct UserService<R: UserRepo> {
    repo: R,
}

impl<R: UserRepo> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
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
        self.repo.create_user(&user).await?;
        Ok(user)
    }

    pub async fn get_user(&self, ident: UserIdentifier) -> Result<User, AppError> {
        self.repo.get_user(ident).await
    }
}

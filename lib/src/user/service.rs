use crate::{
    AppResult,
    database::{Created, Deleted, Updated},
};

use super::{NewUser, User, UserIdentifier, repo::UserRepo};

pub struct UserService {
    repo: Box<dyn UserRepo>,
}

impl UserService {
    pub fn new(repo: impl UserRepo) -> Self {
        Self {
            repo: Box::new(repo),
        }
    }

    fn generate_id(&self) -> String {
        format!("user_{}", uuid::Uuid::new_v4())
    }

    pub async fn create_user(&self, data: NewUser) -> AppResult<Created<User>> {
        let user = User {
            id: self.generate_id(),
            email: data.email,
            email_verified: false,
            password_hash: data.password_hash,
            name: data.name,
            photo_url: data.photo_url,
        };
        self.repo.create_user(user).await
    }

    pub async fn get_user(&self, ident: UserIdentifier) -> AppResult<Updated<User>> {
        self.repo.get_user(ident).await
    }

    pub async fn update_user(&self, user: User) -> AppResult<Updated<User>> {
        self.repo.update_user(user).await
    }

    pub async fn delete_user(&self, ident: UserIdentifier) -> AppResult<Deleted<User>> {
        self.repo.delete_user(ident).await
    }
}

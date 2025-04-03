use dashmap::DashMap;

use crate::AppError;

use super::{repo::UserRepo, User, UserIdentifier};

pub struct UserRepoInMemory {
    data: DashMap<UserIdentifier, DashMap<String, User>>,
}

#[async_trait::async_trait]
impl UserRepo for UserRepoInMemory {
    async fn create_user(&self, user: &User) -> Result<(), AppError> {
        for ident in user.identifiers() {
            if self.data.contains_key(&ident) {
                return Err(AppError::Validation("User already exists".to_string()));
            }
        }
        for ident in user.identifiers() {
            let map = DashMap::new();
            map.insert(ident.to_string(), user.clone());
            self.data.insert(ident, map);
        }
        Ok(())
    }

    async fn get_user(&self, ident: super::UserIdentifier) -> Result<User, AppError> {
        if let Some(map) = self.data.get(&ident) {
            if let Some(user) = map.get(&ident.to_string()) {
                return Ok(user.clone());
            }
        }
        Err(AppError::NotFound)
    }

    async fn update_user(&self, user: &User) -> Result<(), AppError> {
        for ident in user.identifiers() {
            if let Some(map) = self.data.get(&ident) {
                map.insert(ident.to_string(), user.clone());
            }
        }
        Ok(())
    }

    async fn delete_user(&self, ident: UserIdentifier) -> Result<(), AppError> {
        let user = self.get_user(ident.clone()).await?;
        for ident in user.identifiers() {
            self.data.remove(&ident);
        }
        Ok(())
    }
}

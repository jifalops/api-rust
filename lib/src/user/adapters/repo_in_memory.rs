use std::{collections::HashMap, sync::RwLock};

use time::UtcDateTime;

use crate::{
    AppResult,
    database::{Created, Deleted, Updated},
    user::{User, UserError, UserIdentifier, UserRepo},
};

/// In-memory [`UserRepo`], for tests and for running the API without a database.
#[derive(Default)]
pub struct UserRepoInMemory {
    users: RwLock<HashMap<String, Updated<User>>>,
}

impl UserRepoInMemory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Looks up a live (not soft-deleted) record's id.
    fn find_id(&self, ident: &UserIdentifier) -> Option<String> {
        let users = self.users.read().unwrap();
        users
            .values()
            .filter(|u| u.deleted_at.is_none())
            .find(|u| match ident {
                UserIdentifier::Id(id) => &u.model.id == id,
                UserIdentifier::Email(email) => &u.model.email == email,
            })
            .map(|u| u.model.id.clone())
    }
}

#[async_trait::async_trait]
impl UserRepo for UserRepoInMemory {
    async fn create_user(&self, user: User) -> AppResult<Created<User>> {
        for ident in user.identifiers() {
            if self.find_id(&ident).is_some() {
                Err(UserError::UserAlreadyExists(ident.to_string()))?
            }
        }
        let now = UtcDateTime::now();
        let record = Updated {
            model: user,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        let mut users = self.users.write().unwrap();
        users.insert(record.model.id.clone(), record.clone());
        Ok(record.into())
    }

    async fn get_user(&self, ident: UserIdentifier) -> AppResult<Updated<User>> {
        let id = self
            .find_id(&ident)
            .ok_or_else(|| UserError::UserNotFound(ident.to_string()))?;
        let users = self.users.read().unwrap();
        Ok(users.get(&id).cloned().unwrap())
    }

    async fn update_user(&self, user: User) -> AppResult<Updated<User>> {
        let ident = UserIdentifier::Id(user.id.clone());
        let id = self
            .find_id(&ident)
            .ok_or_else(|| UserError::UserNotFound(ident.to_string()))?;
        let mut users = self.users.write().unwrap();
        let record = users.get_mut(&id).unwrap();
        record.model = user;
        record.updated_at = UtcDateTime::now();
        Ok(record.clone())
    }

    async fn delete_user(&self, ident: UserIdentifier) -> AppResult<Deleted<User>> {
        let id = self
            .find_id(&ident)
            .ok_or_else(|| UserError::UserNotFound(ident.to_string()))?;
        let mut users = self.users.write().unwrap();
        let record = users.get_mut(&id).unwrap();
        record.updated_at = UtcDateTime::now();
        record.deleted_at = Some(record.updated_at);
        Ok(record.clone().into())
    }
}

#[cfg(test)]
mod tests {
    use super::UserRepoInMemory;
    use crate::user::testing;

    #[tokio::test]
    async fn test_create_user() {
        testing::test_create_user(UserRepoInMemory::new()).await;
    }

    #[tokio::test]
    async fn test_create_user_rejects_duplicate_email() {
        testing::test_create_user_rejects_duplicate_email(UserRepoInMemory::new()).await;
    }

    #[tokio::test]
    async fn test_get_user() {
        testing::test_get_user(UserRepoInMemory::new()).await;
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        testing::test_get_user_not_found(UserRepoInMemory::new()).await;
    }

    #[tokio::test]
    async fn test_update_user() {
        testing::test_update_user(UserRepoInMemory::new()).await;
    }

    #[tokio::test]
    async fn test_delete_user() {
        testing::test_delete_user(UserRepoInMemory::new()).await;
    }
}

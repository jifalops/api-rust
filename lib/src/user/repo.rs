use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

use crate::{
    AppResult,
    database::{Created, Deleted, Updated},
};

use super::{User, UserIdentifier};

#[async_trait::async_trait]
pub trait UserRepo: Send + Sync + 'static {
    async fn create_user(&self, user: User) -> AppResult<Created<User>>;

    async fn get_user(&self, ident: UserIdentifier) -> AppResult<Updated<User>>;

    async fn update_user(&self, user: User) -> AppResult<Updated<User>>;

    async fn delete_user(&self, ident: UserIdentifier) -> AppResult<Deleted<User>>;
}

/// Row shape of the `users` table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct UserDto {
    pub id: String,
    pub email: String,
    pub email_verified: bool,
    pub password_hash: String,
    pub name: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl From<UserDto> for Updated<User> {
    fn from(dto: UserDto) -> Self {
        Updated {
            model: User {
                id: dto.id,
                email: dto.email,
                email_verified: dto.email_verified,
                password_hash: dto.password_hash,
                name: dto.name,
                photo_url: dto.photo_url,
            },
            created_at: dto.created_at.into(),
            updated_at: dto.updated_at.into(),
            deleted_at: dto.deleted_at.map(Into::into),
        }
    }
}

/// Adapter-agnostic repository tests. Each adapter calls these so every backend
/// is held to the same contract — see `adapters/repo_in_memory.rs` (no database)
/// and `lib/tests/user.rs` (Postgres).
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    pub use super::super::models::testing::new_user;

    use super::UserRepo;
    use crate::user::UserIdentifier;

    pub async fn test_create_user(repo: impl UserRepo) {
        let user = new_user("create@example.com");
        let created = repo.create_user(user.clone()).await.unwrap();
        assert_eq!(created.model, user);
    }

    pub async fn test_create_user_rejects_duplicate_email(repo: impl UserRepo) {
        let user = new_user("duplicate@example.com");
        repo.create_user(user.clone()).await.unwrap();
        let mut other = new_user("duplicate@example.com");
        other.id = format!("{}_other", user.id);
        assert!(repo.create_user(other).await.is_err());
    }

    pub async fn test_get_user(repo: impl UserRepo) {
        let user = new_user("get@example.com");
        repo.create_user(user.clone()).await.unwrap();
        let by_id = repo
            .get_user(UserIdentifier::Id(user.id.clone()))
            .await
            .unwrap();
        assert_eq!(by_id.model, user);
        let by_email = repo
            .get_user(UserIdentifier::Email(user.email.clone()))
            .await
            .unwrap();
        assert_eq!(by_email.model, user);
    }

    pub async fn test_get_user_not_found(repo: impl UserRepo) {
        let result = repo
            .get_user(UserIdentifier::Email("nobody@example.com".to_string()))
            .await;
        assert!(result.is_err());
    }

    pub async fn test_update_user(repo: impl UserRepo) {
        let user = new_user("update@example.com");
        repo.create_user(user.clone()).await.unwrap();
        let mut updated_user = user.clone();
        updated_user.name = Some("New Name".to_string());
        let result = repo.update_user(updated_user.clone()).await.unwrap();
        assert_eq!(result.model, updated_user);
    }

    pub async fn test_delete_user(repo: impl UserRepo) {
        let user = new_user("delete@example.com");
        repo.create_user(user.clone()).await.unwrap();
        let deleted = repo
            .delete_user(UserIdentifier::Id(user.id.clone()))
            .await
            .unwrap();
        assert_eq!(deleted.model, user);
        // Soft-deleted users are no longer readable.
        assert!(repo.get_user(UserIdentifier::Id(user.id)).await.is_err());
    }
}

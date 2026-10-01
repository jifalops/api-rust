use sqlx::{PgPool, query_file_as};

use crate::{
    AppError, AppResult, InfraError,
    database::{Created, Deleted, UNIQUE_VIOLATION, Updated},
    user::{User, UserError, UserIdentifier, UserRepo, repo::UserDto},
};

pub struct UserRepoPostgres {
    pool: PgPool,
}

impl UserRepoPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepo for UserRepoPostgres {
    async fn create_user(&self, user: User) -> AppResult<Created<User>> {
        let dto = query_file_as!(
            UserDto,
            "src/user/adapters/sql/create_user.sql",
            user.id,
            user.email,
            user.email_verified,
            user.password_hash,
            user.name,
            user.photo_url,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some(UNIQUE_VIOLATION) => {
                UserError::UserAlreadyExists(user.email.clone()).into()
            }
            _ => AppError::from(InfraError::Postgres(format!("Error creating user: {e}"))),
        })?;
        Ok(Updated::<User>::from(dto).into())
    }

    async fn get_user(&self, ident: UserIdentifier) -> AppResult<Updated<User>> {
        let id_value = ident.to_string();
        let result = match &ident {
            UserIdentifier::Id(id) => {
                query_file_as!(UserDto, "src/user/adapters/sql/get_user_by_id.sql", id)
                    .fetch_one(&self.pool)
                    .await
            }
            UserIdentifier::Email(email) => {
                query_file_as!(
                    UserDto,
                    "src/user/adapters/sql/get_user_by_email.sql",
                    email
                )
                .fetch_one(&self.pool)
                .await
            }
        };
        let dto = result.map_err(|e| match e {
            sqlx::Error::RowNotFound => UserError::UserNotFound(id_value).into(),
            _ => AppError::from(InfraError::Postgres(format!("Error getting user: {e}"))),
        })?;
        Ok(dto.into())
    }

    async fn update_user(&self, user: User) -> AppResult<Updated<User>> {
        let dto = query_file_as!(
            UserDto,
            "src/user/adapters/sql/update_user.sql",
            user.id,
            user.email_verified,
            user.name,
            user.photo_url,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserError::UserNotFound(user.id.clone()).into(),
            _ => AppError::from(InfraError::Postgres(format!("Error updating user: {e}"))),
        })?;
        Ok(dto.into())
    }

    async fn delete_user(&self, ident: UserIdentifier) -> AppResult<Deleted<User>> {
        let id_value = ident.to_string();
        let result = match &ident {
            UserIdentifier::Id(id) => {
                query_file_as!(UserDto, "src/user/adapters/sql/delete_user_by_id.sql", id)
                    .fetch_one(&self.pool)
                    .await
            }
            UserIdentifier::Email(email) => {
                query_file_as!(
                    UserDto,
                    "src/user/adapters/sql/delete_user_by_email.sql",
                    email
                )
                .fetch_one(&self.pool)
                .await
            }
        };
        let dto = result.map_err(|e| match e {
            sqlx::Error::RowNotFound => UserError::UserNotFound(id_value).into(),
            _ => AppError::from(InfraError::Postgres(format!("Error deleting user: {e}"))),
        })?;
        Ok(Updated::<User>::from(dto).into())
    }
}

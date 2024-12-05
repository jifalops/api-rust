use axum::async_trait;
use serde_json::Value;

use crate::AppError;

use super::repo::UserRepoAdapter;

pub struct UserRepoPostgres;

#[async_trait]
impl UserRepoAdapter for UserRepoPostgres {
    async fn create_user(&self, data: Value) -> Result<(), AppError> {
        todo!()
    }
    async fn get_user_by_id(&self, id: &str) -> Result<Value, AppError> {
        todo!()
    }
    async fn get_user_by_email(&self, email: &str) -> Result<Value, AppError> {
        todo!()
    }
}

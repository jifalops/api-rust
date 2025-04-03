use crate::{app::Repo, user::User, AppError};

use super::Token;

#[async_trait::async_trait]
pub trait AuthRepo: Repo {
    async fn create_token(&self, user: &User) -> Result<Token, AppError>;
}

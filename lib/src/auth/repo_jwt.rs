use crate::{user::User, AppError};

use super::{repo::AuthRepo, Token};

pub struct AuthRepoJwt;

#[async_trait::async_trait]
impl AuthRepo for AuthRepoJwt {
    async fn create_token(&self, user: &User) -> Result<Token, AppError> {
        todo!()
    }
}

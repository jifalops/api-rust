use async_trait::async_trait;

use crate::{user::User, AppError};

use super::{AuthService, Token};

pub struct AuthServiceJwt;

#[async_trait]
impl AuthService for AuthServiceJwt {
    async fn generate_token(&self, user: User) -> Result<Token, AppError> {
        todo!()
    }
}

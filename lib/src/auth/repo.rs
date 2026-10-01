use crate::{AppResult, user::User};

use super::{Token, TokenUser};

#[async_trait::async_trait]
pub trait AuthRepo: Send + Sync + 'static {
    /// Issues a signed token for a user.
    async fn create_token(&self, user: &User) -> AppResult<Token>;

    /// Verifies a token's signature and expiry, returning its claims.
    fn verify(&self, token: &Token) -> AppResult<TokenUser>;
}

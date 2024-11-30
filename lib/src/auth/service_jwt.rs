use axum::async_trait;

use super::AuthService;

#[derive(Clone)]
pub struct AuthServiceJwt;

#[async_trait]
impl AuthService for AuthServiceJwt {
    async fn authenticate(&self, token: &str) -> Result<(), ()> {
        if token == "valid" {
            Ok(())
        } else {
            Err(())
        }
    }
}

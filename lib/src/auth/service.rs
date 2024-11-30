use axum::async_trait;

use crate::app::Service;

#[async_trait]
pub trait AuthService: Service {
    async fn authenticate(&self, token: &str) -> Result<(), ()>;
}

use axum::async_trait;

#[async_trait]
pub trait AuthService {
    async fn authenticate(&self, token: &str) -> Result<(), ()>;
}

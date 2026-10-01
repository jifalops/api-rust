use std::sync::Arc;

use poem_openapi::{OpenApi, payload::Json};

use crate::{
    App, AppResult,
    api::ApiTags,
    user::{UserIdentifier, UserProfile},
};

use super::{BearerAuth, SignInData, SignUpData, Token};

pub struct AuthRouter {
    app: Arc<App>,
}

impl AuthRouter {
    pub fn new(app: Arc<App>) -> Self {
        Self { app }
    }
}

#[OpenApi(tag = "ApiTags::Auth", prefix_path = "/auth")]
impl AuthRouter {
    /// Sign up / Register
    #[oai(path = "/sign_up", method = "post")]
    async fn sign_up(&self, Json(data): Json<SignUpData>) -> AppResult<Json<Token>> {
        let token = self
            .app
            .auth()
            .sign_up(data, self.app.user())
            .await
            .inspect_err(|e| tracing::warn!("Sign up failed: {e}"))?;
        Ok(Json(token))
    }

    /// Sign in / Login
    #[oai(path = "/sign_in", method = "post")]
    async fn sign_in(&self, Json(data): Json<SignInData>) -> AppResult<Json<Token>> {
        let token = self
            .app
            .auth()
            .sign_in(data, self.app.user())
            .await
            .inspect_err(|e| tracing::warn!("Sign in failed: {e}"))?;
        Ok(Json(token))
    }

    /// The signed-in user's own profile.
    #[oai(path = "/me", method = "get")]
    async fn me(&self, BearerAuth(claims): BearerAuth) -> AppResult<Json<UserProfile>> {
        let user = self
            .app
            .user()
            .get_user(UserIdentifier::Id(claims.id))
            .await?;
        Ok(Json(UserProfile::from(&user.model)))
    }
}

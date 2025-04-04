use std::sync::Arc;

use poem_openapi::{ApiResponse, OpenApi, payload::Json};

use crate::{api::ApiTags, app::App};

use super::{SignInData, SignUpData, Token};

pub struct AuthRouter<A: App> {
    pub app: Arc<A>,
}

#[derive(ApiResponse)]
enum AuthResponse {
    #[oai(status = 200)]
    Success(Json<Token>),
    #[oai(status = 401)]
    Unauthorized(Json<String>),
}

#[OpenApi(tag = "ApiTags::Auth", prefix_path = "/auth")]
impl<A: App> AuthRouter<A> {
    /// Sign up / Register
    #[oai(path = "/sign_up", method = "post")]
    async fn sign_up(&self, Json(data): Json<SignUpData>) -> AuthResponse {
        match self.app.auth().sign_up(data, self.app.as_ref()).await {
            Ok(token) => {
                tracing::info!("User signed up successfully: {:?}", token);
                AuthResponse::Success(Json(token))
            }
            Err(e) => {
                tracing::error!("Sign up error: {:?}", e);
                AuthResponse::Unauthorized(Json(e.to_string()))
            }
        }
    }

    /// Sign in / Login
    #[oai(path = "/sign_in", method = "post")]
    async fn sign_in(&self, Json(data): Json<SignInData>) -> AuthResponse {
        match self.app.auth().sign_in(data, self.app.as_ref()).await {
            Ok(token) => {
                tracing::info!("User signed in successfully: {:?}", token);
                AuthResponse::Success(Json(token))
            }
            Err(e) => {
                tracing::error!("Sign in error: {:?}", e);
                AuthResponse::Unauthorized(Json(e.to_string()))
            }
        }
    }
}

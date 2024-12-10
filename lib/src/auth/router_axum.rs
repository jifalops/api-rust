use std::sync::Arc;

use aide::axum::{routing::post, ApiRouter};
use axum::extract::State;

use crate::{app::App, extractors::Json, AppError};

use super::{router, SignInData, SignUpData, Token};

pub fn create_router<A: App>() -> ApiRouter<Arc<A>> {
    ApiRouter::new()
        .api_route("/sign-up", post(sign_up))
        .api_route("/sign-in", post(sign_in))
}

async fn sign_up<A: App>(
    State(app): State<Arc<A>>,
    Json(data): Json<SignUpData>,
) -> Result<Json<Token>, AppError> {
    let token = router::sign_up(data, app.as_ref()).await?;
    Ok(Json(token))
}

async fn sign_in<A: App>(
    State(app): State<Arc<A>>,
    Json(data): Json<SignInData>,
) -> Result<Json<Token>, AppError> {
    let token = router::sign_in(data, app.as_ref()).await?;
    Ok(Json(token))
}

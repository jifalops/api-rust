use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};

use crate::{app::App, AppError};

use super::{SignInData, SignUpData, Token};

pub fn create_router<A: App>() -> Router<Arc<A>> {
    Router::new()
        .route("/sign-up", post(sign_up))
        .route("/sign-in", post(sign_in))
}

async fn sign_up<A: App>(
    State(app): State<Arc<A>>,
    Json(data): Json<SignUpData>,
) -> Result<Json<Token>, AppError> {
    let token = app.auth().sign_up(data, app.as_ref()).await?;
    Ok(Json(token))
}

async fn sign_in<A: App>(
    State(app): State<Arc<A>>,
    Json(data): Json<SignInData>,
) -> Result<Json<Token>, AppError> {
    let token = app.auth().sign_in(data, app.as_ref()).await?;
    Ok(Json(token))
}

use std::sync::Arc;

use api_rust::{
    app::{App, NewApp},
    auth::{AuthService, AuthServiceJwt},
};
use axum::{extract::State, response::Html, routing::get, Json, Router};
use tracing::debug;

#[tokio::main]
async fn main() {
    #[cfg(not(feature = "lambda"))]
    {
        use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();
    }
    #[cfg(feature = "lambda")]
    {
        lambda_http::tracing::init_default_subscriber();
    }

    let app = NewApp {
        auth: AuthServiceJwt,
    };

    let router = Router::new()
        .route("/", get(handler))
        .route("/auth", get(auth))
        .with_state(Arc::new(app));

    #[cfg(not(feature = "lambda"))]
    {
        debug!("DEV MODE");
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }

    #[cfg(feature = "lambda")]
    {
        debug!("LAMBDA MODE");
        lambda_http::run(router).await.unwrap();
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn auth<A: App>(State(app): State<Arc<A>>) -> Json<&'static str> {
    match app.auth().authenticate("valid").await {
        Ok(()) => Json("authenticated"),
        Err(()) => Json("unauthenticated"),
    }
}

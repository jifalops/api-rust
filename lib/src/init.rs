use std::sync::Arc;

use axum::{routing::get, Json, Router};
use oasgen::oasgen;
use serde_json::{json, Value};
use tracing::debug;

use crate::{
    app::NewApp,
    auth::{self, AuthServiceJwt},
    user::{UserRepoPostgres, UserService},
};

pub async fn initialize() {
    setup_tracing();

    let app = NewApp {
        auth: AuthServiceJwt,
        user: UserService::new(UserRepoPostgres),
    };

    let server = oasgen::Server::axum()
        // TODO(multiple routers): https://github.com/kurtbuilds/oasgen/issues/20
        .route_json_spec("/openapi.json")
        .swagger_ui("/docs/");

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/auth", auth::router_axum::create_router())
        .with_state(Arc::new(app));

    start_api_server(router).await;
}

fn setup_tracing() {
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
}

async fn start_api_server(router: Router) {
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

#[oasgen]
async fn hello_world() -> Json<Value> {
    Json(json!({"message": "Hello, World!"}))
}

use std::sync::Arc;

use axum::{response::Html, routing::get, Router};
use tracing::debug;

use crate::{
    app::NewApp,
    auth::{self, AuthRepoJwt, AuthService},
    user::{UserRepoPostgres, UserService},
};

pub async fn initialize() {
    setup_tracing();

    let app = NewApp {
        auth: AuthService::new(AuthRepoJwt),
        user: UserService::new(UserRepoPostgres),
    };

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/auth", auth::create_router())
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

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

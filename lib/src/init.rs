use std::sync::Arc;

use poem::{Route, listener::TcpListener};
use poem_openapi::OpenApiService;

use crate::{
    app::NewApp,
    auth::{AuthRepoJwt, AuthRouter, AuthService},
    user::{UserRepoPostgres, UserService},
};

pub async fn initialize() {
    setup_tracing();

    let app = Arc::new(NewApp {
        auth: AuthService::new(AuthRepoJwt),
        user: UserService::new(UserRepoPostgres),
    });

    let routers = AuthRouter { app: app.clone() };
    let api = OpenApiService::new(routers, "API", "1.0");
    let ui = api.stoplight_elements();
    let spec = api.spec_endpoint();
    let routes = Route::new()
        .nest("/", ui)
        .nest("/api", api)
        .nest("/spec", spec);

    #[cfg(not(feature = "lambda"))]
    {
        let _ = poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
            .run(routes)
            .await;
    }
    #[cfg(feature = "lambda")]
    {
        let _ = poem_lambda::run(routes);
    }
}

fn setup_tracing() {
    #[cfg(not(feature = "lambda"))]
    {
        use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
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

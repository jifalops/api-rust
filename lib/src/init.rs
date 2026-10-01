use std::sync::Arc;

use jsonwebtoken::Algorithm;
#[cfg(not(feature = "lambda"))]
use poem::listener::TcpListener;
use poem::{
    EndpointExt, Response, Route,
    error::ResponseError,
    get, handler,
    middleware::{AddData, Cors, Tracing},
};
use poem_openapi::OpenApiService;
#[cfg(not(feature = "lambda"))]
use tracing::info;

use crate::{
    App, AppError, InfraResult,
    auth::{AuthRepoJwt, AuthRouter, AuthService},
    config,
    error_reporting::ErrorReportingService,
    postgres,
    user::{UserRepoPostgres, UserService},
};

#[handler]
fn health() -> &'static str {
    "ok"
}

/// Cross-cutting infrastructure, built once at startup.
struct Infra {
    postgres_pool: sqlx::Pool<sqlx::Postgres>,
}

async fn setup_infra() -> InfraResult<Infra> {
    Ok(Infra {
        postgres_pool: postgres::connect().await?,
    })
}

pub async fn initialize() {
    setup_tracing();

    let error_reporting = Arc::new(ErrorReportingService::new());

    let infra = match setup_infra().await {
        Ok(infra) => infra,
        Err(e) => {
            error_reporting.report_app_error(&AppError::from(e.clone()));
            panic!("Failed to set up infrastructure: {e}");
        }
    };

    let auth = Arc::new(AuthService::new(AuthRepoJwt::new(
        &config::JWT_SECRET,
        Algorithm::HS256,
        config::token_ttl(),
    )));
    let user = Arc::new(UserService::new(UserRepoPostgres::new(
        infra.postgres_pool.clone(),
    )));
    let app = Arc::new(App::new(auth.clone(), user));

    let api = OpenApiService::new(AuthRouter::new(app), "API", "1.0");
    let ui = api.stoplight_elements();
    let spec = api.spec_endpoint();

    let error_reporting_catch = error_reporting.clone();
    let routes = Route::new()
        .at("/health", get(health))
        .nest("/", ui)
        .nest(
            "/api",
            api.with(AddData::new(auth))
                .with(AddData::new(error_reporting)),
        )
        .nest("/spec", spec)
        .catch_all_error(move |err| {
            let service = error_reporting_catch.clone();
            async move {
                if let Some(app_err) = err.downcast_ref::<AppError>() {
                    service.report_app_error(app_err);
                    Response::builder().status(app_err.status()).finish()
                } else {
                    service.report_unknown_error(&err);
                    Response::builder().status(err.status()).finish()
                }
            }
        })
        .with(Cors::new())
        .with(Tracing);

    #[cfg(not(feature = "lambda"))]
    {
        let addr = format!("0.0.0.0:{}", config::port());
        info!("Listening on {addr}");
        poem::Server::new(TcpListener::bind(addr))
            .run_with_graceful_shutdown(routes, shutdown_signal(), None)
            .await
            .expect("Server failed");
    }
    #[cfg(feature = "lambda")]
    {
        poem_lambda::run(routes)
            .await
            .expect("Lambda runtime failed");
    }
}

#[cfg(not(feature = "lambda"))]
async fn shutdown_signal() {
    use tokio::signal;
    let ctrl_c = signal::ctrl_c();
    let mut sigterm =
        signal::unix::signal(signal::unix::SignalKind::terminate()).expect("SIGTERM handler");
    tokio::select! {
        _ = ctrl_c => {}
        _ = sigterm.recv() => {}
    }
    info!("Shutdown signal received");
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

//! Environment configuration, validated once at startup.
//!
//! Required variables are resolved eagerly so a misconfigured deployment fails
//! on boot rather than on the first request that needs them.

use std::sync::LazyLock;

/// Postgres connection string. `DATABASE_URL` is the name `sqlx-cli`,
/// `sqlx::migrate!`, and `#[sqlx::test]` all expect, so the app reads the
/// same variable rather than introducing a second name.
pub static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

/// Secret used to sign and verify JWTs.
pub static JWT_SECRET: LazyLock<String> =
    LazyLock::new(|| std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"));

/// Port to bind the server to. Defaults to 3000.
pub fn port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000)
}

/// How long an issued token stays valid.
pub fn token_ttl() -> time::Duration {
    time::Duration::hours(
        std::env::var("TOKEN_TTL_HOURS")
            .ok()
            .and_then(|h| h.parse().ok())
            .unwrap_or(24),
    )
}

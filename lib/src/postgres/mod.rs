use sqlx::{PgPool, Pool, Postgres};

use crate::{InfraError, InfraResult, config::DATABASE_URL};

/// Connects to Postgres and applies any pending migrations.
pub async fn connect() -> InfraResult<Pool<Postgres>> {
    let pool = PgPool::connect(&DATABASE_URL)
        .await
        .map_err(|e| InfraError::Postgres(format!("Failed to connect to Postgres: {e}")))?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| InfraError::Postgres(format!("Failed to run migrations: {e}")))?;
    Ok(pool)
}

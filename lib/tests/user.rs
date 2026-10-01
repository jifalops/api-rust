//! Postgres-backed contract tests for `UserRepo`.
//!
//! These run the same assertions as the in-memory adapter's unit tests, against a
//! real database. `#[sqlx::test]` provisions an ephemeral, migrated database per
//! test, so they need `DATABASE_URL` and are gated behind the `testing` feature.

use api_rust::user::{UserRepoPostgres, testing};
use sqlx::PgPool;

#[sqlx::test]
async fn test_create_user(pool: PgPool) {
    testing::test_create_user(UserRepoPostgres::new(pool)).await;
}

#[sqlx::test]
async fn test_create_user_rejects_duplicate_email(pool: PgPool) {
    testing::test_create_user_rejects_duplicate_email(UserRepoPostgres::new(pool)).await;
}

#[sqlx::test]
async fn test_get_user(pool: PgPool) {
    testing::test_get_user(UserRepoPostgres::new(pool)).await;
}

#[sqlx::test]
async fn test_get_user_not_found(pool: PgPool) {
    testing::test_get_user_not_found(UserRepoPostgres::new(pool)).await;
}

#[sqlx::test]
async fn test_update_user(pool: PgPool) {
    testing::test_update_user(UserRepoPostgres::new(pool)).await;
}

#[sqlx::test]
async fn test_delete_user(pool: PgPool) {
    testing::test_delete_user(UserRepoPostgres::new(pool)).await;
}

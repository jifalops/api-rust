use serde::{Deserialize, Serialize};
use time::UtcDateTime;

/// Postgres SQLSTATE for a unique constraint violation.
pub const UNIQUE_VIOLATION: &str = "23505";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WithId<T> {
    pub model: T,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Created<T> {
    pub model: T,
    pub created_at: UtcDateTime,
    pub updated_at: Option<UtcDateTime>,
    pub deleted_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Updated<T> {
    pub model: T,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub deleted_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Deleted<T> {
    pub model: T,
    pub deleted_at: UtcDateTime,
    pub created_at: UtcDateTime,
    pub updated_at: Option<UtcDateTime>,
}

impl<T> From<Updated<T>> for Created<T> {
    fn from(updated: Updated<T>) -> Self {
        Created {
            model: updated.model,
            created_at: updated.created_at,
            updated_at: Some(updated.updated_at),
            deleted_at: updated.deleted_at,
        }
    }
}

impl<T> From<Updated<T>> for Deleted<T> {
    fn from(updated: Updated<T>) -> Self {
        Deleted {
            model: updated.model,
            deleted_at: updated.updated_at,
            created_at: updated.created_at,
            updated_at: Some(updated.updated_at),
        }
    }
}

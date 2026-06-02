use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(ts_rs::TS, Debug, Clone, Serialize, Deserialize, FromRow)]
#[ts(export)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_email(
        db: &sqlx::PgPool,
        email: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, created_at, updated_at
             FROM users WHERE email = $1",
            email
        )
        .fetch_optional(db)
        .await
    }

    pub async fn find_by_id(db: &sqlx::PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, created_at, updated_at
             FROM users WHERE id = $1",
            id
        )
        .fetch_optional(db)
        .await
    }

    pub async fn create_oauth_user(
        db: &sqlx::PgPool,
        email: &str,
        _given_name: &str,
        _family_name: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (email, password_hash)
             VALUES ($1, '')
             RETURNING id, email, password_hash, created_at, updated_at",
            email
        )
        .fetch_one(db)
        .await
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(ts_rs::TS, Debug, Clone, Serialize, Deserialize, FromRow)]
#[ts(export)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_email(
        db: &sqlx::PgPool,
        email: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, first_name, last_name, password_hash, created_at, updated_at
             FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(db)
        .await
    }

    pub async fn find_by_id(db: &sqlx::PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, first_name, last_name, password_hash, created_at, updated_at
             FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await
    }

    pub async fn create(
        db: &sqlx::PgPool,
        email: &str,
        password_hash: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash)
             VALUES ($1, $2)
             RETURNING id, email, password_hash, created_at, updated_at",
        )
        .bind(email)
        .bind(password_hash)
        .fetch_one(db)
        .await
    }

    pub async fn email_exists(db: &sqlx::PgPool, email: &str) -> Result<bool, sqlx::Error> {
        let result: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(db)
            .await?;
        Ok(result.is_some())
    }
}

impl RefreshToken {
    pub async fn create(
        db: &sqlx::PgPool,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, RefreshToken>(
            "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
             VALUES ($1, $2, $3)
             RETURNING id, user_id, token_hash, expires_at, created_at",
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .fetch_one(db)
        .await
    }

    pub async fn find_by_hash(
        db: &sqlx::PgPool,
        token_hash: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, RefreshToken>(
            "SELECT id, user_id, token_hash, expires_at, created_at
             FROM refresh_tokens WHERE token_hash = $1",
        )
        .bind(token_hash)
        .fetch_optional(db)
        .await
    }

    pub async fn delete_by_user_id(db: &sqlx::PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn delete_by_hash(db: &sqlx::PgPool, token_hash: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM refresh_tokens WHERE token_hash = $1")
            .bind(token_hash)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn delete_expired(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM refresh_tokens WHERE expires_at < NOW()")
            .execute(db)
            .await?;
        Ok(())
    }
}

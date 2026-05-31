use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub async fn create(
        db: &sqlx::PgPool,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            RefreshToken,
            "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
             VALUES ($1, $2, $3)
             RETURNING id, user_id, token_hash, expires_at, created_at",
            user_id,
            token_hash,
            expires_at
        )
        .fetch_one(db)
        .await
    }

    pub async fn find_by_hash(
        db: &sqlx::PgPool,
        token_hash: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            RefreshToken,
            "SELECT id, user_id, token_hash, expires_at, created_at
             FROM refresh_tokens WHERE token_hash = $1",
            token_hash
        )
        .fetch_optional(db)
        .await
    }

    pub async fn delete_by_user_id(db: &sqlx::PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn delete_by_hash(db: &sqlx::PgPool, token_hash: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM refresh_tokens WHERE token_hash = $1",
            token_hash
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn delete_expired(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE expires_at < NOW()")
            .execute(db)
            .await?;
        Ok(())
    }
}

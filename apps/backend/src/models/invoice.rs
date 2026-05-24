use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub customer_id: Uuid,
    pub cost: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInvoice {
    pub customer_id: Uuid,
    pub cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInvoice {
    pub customer_id: Option<Uuid>,
    pub cost: Option<f64>,
}

impl Invoice {
    pub async fn find_by_id(db: &sqlx::PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Invoice>(
            "SELECT id, user_id, customer_id, cost, created_at, updated_at 
             FROM invoices WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await
    }

    pub async fn find_by_user(
        db: &sqlx::PgPool,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        sqlx::query_as::<_, Invoice>(
            "SELECT id, user_id, customer_id, cost, created_at, updated_at 
             FROM invoices WHERE user_id = $1 
             ORDER BY created_at DESC 
             LIMIT $2 OFFSET $3"
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(db)
        .await
    }

    pub async fn find_by_customer(
        db: &sqlx::PgPool,
        user_id: Uuid,
        customer_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Invoice>(
            "SELECT id, user_id, customer_id, cost, created_at, updated_at 
             FROM invoices WHERE user_id = $1 AND customer_id = $2 
             ORDER BY created_at DESC"
        )
        .bind(user_id)
        .bind(customer_id)
        .fetch_all(db)
        .await
    }

    pub async fn create(
        db: &sqlx::PgPool,
        user_id: Uuid,
        customer_id: Uuid,
        cost: f64,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Invoice>(
            "INSERT INTO invoices (id, user_id, customer_id, cost, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())
             RETURNING id, user_id, customer_id, cost, created_at, updated_at"
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(customer_id)
        .bind(cost)
        .fetch_one(db)
        .await
    }

    pub async fn update(
        db: &sqlx::PgPool,
        id: Uuid,
        customer_id: Option<Uuid>,
        cost: Option<f64>,
    ) -> Result<Option<Self>, sqlx::Error> {
        if customer_id.is_none() && cost.is_none() {
            return Self::find_by_id(db, id).await;
        }

        let current = Self::find_by_id(db, id).await?;
        if current.is_none() {
            return Ok(None);
        }
        let current = current.unwrap();

        let new_customer_id = customer_id.unwrap_or(current.customer_id);
        let new_cost = cost.unwrap_or(current.cost);

        sqlx::query_as::<_, Invoice>(
            "UPDATE invoices 
             SET customer_id = $1, cost = $2, updated_at = NOW()
             WHERE id = $3
             RETURNING id, user_id, customer_id, cost, created_at, updated_at"
        )
        .bind(new_customer_id)
        .bind(new_cost)
        .bind(id)
        .fetch_optional(db)
        .await
    }

    pub async fn delete(db: &sqlx::PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM invoices WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count_by_user(db: &sqlx::PgPool, user_id: Uuid) -> Result<i64, sqlx::Error> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT COUNT(*) FROM invoices WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(db)
        .await?;

        Ok(result.map(|(c,)| c).unwrap_or(0))
    }
}
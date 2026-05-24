use axum::{
    extract::{Extension, Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::ApiError,
    middleware::auth::AuthenticatedUser,
    models::invoice::Invoice,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListInvoicesQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ListByCustomerQuery {
    customer_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct InvoiceResponse {
    pub id: String,
    pub user_id: String,
    pub customer_id: String,
    pub cost: f64,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Invoice> for InvoiceResponse {
    fn from(invoice: Invoice) -> Self {
        Self {
            id: invoice.id.to_string(),
            user_id: invoice.user_id.to_string(),
            customer_id: invoice.customer_id.to_string(),
            cost: invoice.cost,
            created_at: invoice.created_at.to_rfc3339(),
            updated_at: invoice.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InvoiceListResponse {
    pub invoices: Vec<InvoiceResponse>,
    pub total: i64,
}

pub async fn create_invoice(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Json(payload): Json< crate::models::invoice::CreateInvoice>,
) -> Result<Json<InvoiceResponse>, ApiError> {
    let invoice = Invoice::create(
        &state.db,
        auth_user.user_id,
        payload.customer_id,
        payload.cost,
    )
    .await?;

    Ok(Json(InvoiceResponse::from(invoice)))
}

pub async fn list_invoices(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Query(query): Query<ListInvoicesQuery>,
) -> Result<Json<InvoiceListResponse>, ApiError> {
    let invoices = Invoice::find_by_user(
        &state.db,
        auth_user.user_id,
        query.limit,
        query.offset,
    )
    .await?;

    let total = Invoice::count_by_user(&state.db, auth_user.user_id).await?;

    Ok(Json(InvoiceListResponse {
        invoices: invoices.into_iter().map(InvoiceResponse::from).collect(),
        total,
    }))
}

pub async fn get_invoice(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(invoice_id): Path<Uuid>,
) -> Result<Json<InvoiceResponse>, ApiError> {
    let invoice = Invoice::find_by_id(&state.db, invoice_id)
        .await?
        .ok_or(ApiError::InvoiceNotFound)?;

    if invoice.user_id != auth_user.user_id {
        return Err(ApiError::InvoiceNotFound);
    }

    Ok(Json(InvoiceResponse::from(invoice)))
}

pub async fn list_invoices_by_customer(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Query(query): Query<ListByCustomerQuery>,
) -> Result<Json<Vec<InvoiceResponse>>, ApiError> {
    let invoices = Invoice::find_by_customer(&state.db, auth_user.user_id, query.customer_id).await?;

    Ok(Json(invoices.into_iter().map(InvoiceResponse::from).collect()))
}

pub async fn update_invoice(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(invoice_id): Path<Uuid>,
    Json(payload): Json<crate::models::invoice::UpdateInvoice>,
) -> Result<Json<InvoiceResponse>, ApiError> {
    let existing = Invoice::find_by_id(&state.db, invoice_id)
        .await?
        .ok_or(ApiError::InvoiceNotFound)?;

    if existing.user_id != auth_user.user_id {
        return Err(ApiError::InvoiceNotFound);
    }

    let invoice = Invoice::update(
        &state.db,
        invoice_id,
        payload.customer_id,
        payload.cost,
    )
    .await?
    .ok_or(ApiError::InvoiceNotFound)?;

    Ok(Json(InvoiceResponse::from(invoice)))
}

pub async fn delete_invoice(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(invoice_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let existing = Invoice::find_by_id(&state.db, invoice_id)
        .await?
        .ok_or(ApiError::InvoiceNotFound)?;

    if existing.user_id != auth_user.user_id {
        return Err(ApiError::InvoiceNotFound);
    }

    Invoice::delete(&state.db, invoice_id).await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}
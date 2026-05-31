use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{handlers::invoice, state::AppState};

pub fn invoice_routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(invoice::create_invoice).with_state(state.clone()))
        .route("/", get(invoice::list_invoices).with_state(state.clone()))
        .route(
            "/customer",
            get(invoice::list_invoices_by_customer).with_state(state.clone()),
        )
        .route("/{id}", get(invoice::get_invoice).with_state(state.clone()))
        .route(
            "/{id}",
            put(invoice::update_invoice).with_state(state.clone()),
        )
        .route(
            "/{id}",
            delete(invoice::delete_invoice).with_state(state.clone()),
        )
        .with_state(state)
}

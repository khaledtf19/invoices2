use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::{handlers::invoice, middleware::auth::jwt_auth_middleware, state::AppState};

pub fn invoice_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            post(invoice::create_invoice).layer(middleware::from_fn_with_state(
                state.clone(),
                jwt_auth_middleware,
            )),
        )
        .route(
            "/",
            get(invoice::list_invoices).layer(middleware::from_fn_with_state(
                state.clone(),
                jwt_auth_middleware,
            )),
        )
        .route(
            "/customer",
            get(invoice::list_invoices_by_customer).layer(middleware::from_fn_with_state(
                state.clone(),
                jwt_auth_middleware,
            )),
        )
        .route(
            "/{id}",
            get(invoice::get_invoice).layer(middleware::from_fn_with_state(
                state.clone(),
                jwt_auth_middleware,
            )),
        )
        .route(
            "/{id}",
            put(invoice::update_invoice).layer(middleware::from_fn_with_state(
                state.clone(),
                jwt_auth_middleware,
            )),
        )
        .route(
            "/{id}",
            delete(invoice::delete_invoice).layer(middleware::from_fn_with_state(
                state.clone(),
                jwt_auth_middleware,
            )),
        )
        .with_state(state)
}

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{handlers::auth, middleware::auth::jwt_auth_middleware, state::AppState};

pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
        .route(
            "/logout",
            post(auth::logout).layer(middleware::from_fn_with_state(
                state.clone(),
                jwt_auth_middleware,
            )),
        )
        .route(
            "/me",
            get(auth::me).layer(middleware::from_fn_with_state(
                state.clone(),
                jwt_auth_middleware,
            )),
        )
        .with_state(state)
}

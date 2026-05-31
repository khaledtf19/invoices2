use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{auth, oauth},
    state::AppState,
};

pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::me))
        .route("/google-auth", get(oauth::google_auth))
        .route("/google/callback", get(oauth::google_callback))
        .with_state(state)
}

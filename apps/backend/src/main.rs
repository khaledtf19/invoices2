mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};

use crate::{db::pool::create_pool, routes::auth::auth_routes, state::AppState};

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let config = config::Config::from_env()?;

    let pool = create_pool(&config.database_url).await?;

    let state = AppState::new(pool, config);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/auth", auth_routes(state.clone()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server is running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

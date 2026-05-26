mod config;
mod db;
pub mod error;
mod handlers;
mod middleware;
mod models;
mod response;
mod routes;
mod services;
mod state;

use axum::{Router, http::Method, routing::get};
use serde_json::Value;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::{
    db::pool::create_pool, routes::auth::auth_routes, routes::invoice::invoice_routes,
    state::AppState,
};

async fn health_check() -> Result<response::ApiResponse<Value>, error::ApiError> {
    Ok(response::ApiResponse::ok(
        serde_json::json!({ "status": "ok" }),
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env()?;

    let pool = create_pool(&config.database_url).await?;

    sqlx::migrate!("src/db/migrations").run(&pool).await?;

    let state = AppState::new(pool, config);

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(["http://localhost:5173"
            .parse()
            .unwrap()]))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            "content-type".parse().unwrap(),
            "authorization".parse().unwrap(),
            "cookie".parse().unwrap(),
        ])
        .allow_credentials(true);

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/invoices", invoice_routes(state.clone()))
        .layer(cors);
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: std::net::SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Server is running on http://0.0.0.0:{}", port);
    axum::serve(listener, app).await?;

    Ok(())
}

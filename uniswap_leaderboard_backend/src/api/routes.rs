use super::handlers::{leaderboard_handler, trader_handler};
use super::sync::sync_handler;
use axum::{
    routing::{get, post},
    Router,
    Json,
};
use reqwest::Client;
use serde_json::json;
use sqlx::PgPool;

pub fn create_router(db_pool: PgPool, http_client: Client) -> Router {
    let app_state = (db_pool, http_client);

    Router::new()
        .route("/health", get(health_check))
        .route("/sync", post(sync_handler).with_state(app_state.clone()))
        .route("/leaderboard", get(leaderboard_handler).with_state(app_state.0.clone()))
        .route("/trader/{address}", get(trader_handler).with_state(app_state.0))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "uniswap_leaderboard_backend",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
// src/api/routes.rs

use super::handlers::{leaderboard_handler, trader_handler};
use super::sync::sync_handler;
use axum::{
    routing::{get, post},
    Router,
};
use reqwest::Client;
use sqlx::PgPool;

pub fn create_router(db_pool: PgPool, http_client: Client) -> Router {
    // This state can be cloned and passed to different route handlers.
    let app_state = (db_pool, http_client);

    Router::new()
        .route("/sync", post(sync_handler).with_state(app_state.clone()))
        .route("/leaderboard", get(leaderboard_handler).with_state(app_state.clone().0))
        .route("/trader/{address}", get(trader_handler).with_state(app_state.0))
}
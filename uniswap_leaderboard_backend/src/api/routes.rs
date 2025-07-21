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
    // Our full application state is a tuple: (PgPool, Client)
    let app_state = (db_pool, http_client);

    Router::new()
        // CORRECT: Provide ONLY the http_client (app_state.1) to the sync_handler
        .route("/sync", post(sync_handler).with_state(app_state.1))
        
        // Provide ONLY the db_pool (app_state.0) to the other handlers
        .route("/leaderboard", get(leaderboard_handler).with_state(app_state.0.clone()))
        .route("/trader/{address}", get(trader_handler).with_state(app_state.0))
}
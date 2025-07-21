pub mod api;
pub mod models;

use axum::Router;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Load environment variables from .env (this will fail on Render, which is fine)
    let _ = dotenvy::dotenv();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Set up the database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("Failed to create database pool");

    println!("✅ Database connection successful");

    // Create a single reqwest::Client to be shared across the application
    let http_client = Client::new();

    // Get our API routes from the other module
    let api_router = api::routes::create_router(db_pool.clone(), http_client);

    // Create the main application router and nest our API routes under /api/v1
    let app = Router::new()
        .nest("/api/v1", api_router);

    // Use PORT environment variable if available (for Render), otherwise default to 3000
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
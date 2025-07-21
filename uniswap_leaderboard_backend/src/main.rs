pub mod api;
pub mod models;

use axum::Router;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenvy::dotenv().expect("Failed to read .env file");

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

    // Start the server
    let addr = "0.0.0.0:3000";
    println!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
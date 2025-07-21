pub mod api;
pub mod models;

use axum::Router;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting Uniswap Leaderboard Backend...");
    
    // Load environment variables from .env (this will fail on Render, which is fine)
    let _ = dotenvy::dotenv();
    
    // Get PORT environment variable, default to 3000
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    println!("PORT: {}", port);
    
    // Validate DATABASE_URL is present
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => {
            println!("DATABASE_URL found");
            url
        }
        Err(e) => {
            eprintln!("ERROR: DATABASE_URL not set: {}", e);
            std::process::exit(1);
        }
    };

    println!("Connecting to database...");
    
    // Set up the database connection pool
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("Database connection successful");
            pool
        }
        Err(e) => {
            eprintln!("ERROR: Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Create HTTP client
    let http_client = Client::new();

    // Set up API routes
    let api_router = api::routes::create_router(db_pool.clone(), http_client);

    // Create the main application router
    let app = Router::new()
        .nest("/api/v1", api_router);

    // Bind to the server
    let addr = format!("0.0.0.0:{}", port);
    println!("Binding to: {}", addr);
    
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("Successfully bound to {}", addr);
            listener
        }
        Err(e) => {
            eprintln!("ERROR: Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };
    
    println!("Server listening on http://{}", addr);
    
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("ERROR: Server crashed: {}", e);
        std::process::exit(1);
    }
}
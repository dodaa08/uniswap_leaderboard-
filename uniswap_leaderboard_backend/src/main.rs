pub mod api;
pub mod models;

use axum::{Router, Json, routing::get};
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    println!("Starting Uniswap Leaderboard Backend...");
    
    // Load environment variables from .env (this will fail on Render, which is fine)
    let _ = dotenvy::dotenv();
    
    // Get PORT environment variable - Render will set this automatically
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    println!("Using PORT: {}", port);

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

    // Configure CORS to allow frontend on port 3000
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse().unwrap(),
            "https://localhost:3000".parse().unwrap(),
            "http://127.0.0.1:3000".parse().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    // Create the main application router with CORS support
    let app = Router::new()
        .route("/", get(|| async { "Uniswap Leaderboard Backend - API available at /api/v1/" }))
        .route("/health", get(|| async { 
            Json(json!({
                "status": "healthy",
                "service": "uniswap_leaderboard_backend",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }))
        .nest("/api/v1", api_router)
        .layer(cors);

    
    // Bind to 0.0.0.0 (not localhost) as required by Render
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
    
    println!("Server listening on {}", addr);
    println!("CORS enabled for frontend on port 3000");
    println!("API endpoints:");
    println!("  GET  /health - Health check");
    println!("  GET  /api/v1/health - API health check");
    println!("  GET  /api/v1/leaderboard - Get top traders");
    println!("  GET  /api/v1/trader/{{address}} - Get trader details");
    println!("  POST /api/v1/sync - Sync data from The Graph");
    
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("ERROR: Server crashed: {}", e);
        std::process::exit(1);
    }
}
use axum::Router;
use reqwest::Client;
use sqlx::PgPool;
use std::env;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod models;

#[tokio::main]
async fn main() {
    println!("=== UNISWAP LEADERBOARD BACKEND STARTING ===");
    
    // Print all environment variables for debugging
    println!("Environment variables:");
    for (key, value) in env::vars() {
        if key.contains("PORT") || key.contains("RENDER") || key.contains("DATABASE") {
            println!("  {}: {}", key, value);
        }
    }
    
    // Get PORT - this is critical for Render
    let port = env::var("PORT").unwrap_or_else(|_| {
        println!("WARNING: PORT not set, defaulting to 3000");
        "3000".to_string()
    });
    
    println!("Attempting to use PORT: {}", port);
    
    // Get database URL
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        println!("WARNING: DATABASE_URL not set, using default postgres URL");
        "postgres://localhost/uniswap_leaderboard".to_string()
    });
    
    println!("Connecting to database...");
    
    // Create database connection pool
    let db_pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            println!("SUCCESS: Connected to database");
            pool
        }
        Err(e) => {
            eprintln!("FAILED to connect to database: {}", e);
            eprintln!("Creating minimal router without database functionality");
            return start_minimal_server(port).await;
        }
    };
    
    // Create HTTP client for external API calls
    let http_client = Client::new();
    
    // Create router with API routes
    let router = api::routes::create_router(db_pool, http_client);
    
    // Add CORS support
    let app = Router::new()
        .nest("/api/v1", router)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    
    // Critical: bind to 0.0.0.0, not 127.0.0.1
    let bind_addr = format!("0.0.0.0:{}", port);
    println!("Attempting to bind to: {}", bind_addr);
    
    // Try to bind
    let listener = match tokio::net::TcpListener::bind(&bind_addr).await {
        Ok(listener) => {
            println!("SUCCESS: Bound to {}", bind_addr);
            listener
        }
        Err(e) => {
            eprintln!("FAILED to bind to {}: {}", bind_addr, e);
            panic!("Cannot bind to address");
        }
    };
    
    println!("Full server starting on {}", bind_addr);
    println!("Available endpoints:");
    println!("  GET  /api/v1/health         - Health check");
    println!("  GET  /api/v1/leaderboard    - Get leaderboard data");
    println!("  POST /api/v1/sync           - Sync data from Uniswap");
    println!("  GET  /api/v1/trader/:address - Get trader details");
    
    // Start the server
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }
}

async fn start_minimal_server(port: String) {
    use axum::routing::get;
    
    println!("Starting minimal server without database...");
    
    let app = Router::new()
        .route("/", get(|| async { "Hello from Minimal Rust Server on Render!" }))
        .route("/health", get(|| async { "OK" }))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    
    let bind_addr = format!("0.0.0.0:{}", port);
    println!("Minimal server binding to: {}", bind_addr);
    
    let listener = tokio::net::TcpListener::bind(&bind_addr).await
        .expect("Failed to bind minimal server");
    
    println!("Minimal server starting on {}", bind_addr);
    
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Minimal server error: {}", e);
    }
}
use axum::Router;
use reqwest::Client;
use sqlx::PgPool;
use std::env;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod models;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://localhost/uniswap_leaderboard".to_string()
    });
    
    let db_pool = match PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(_) => {
            println!("Database connection failed, starting minimal server");
            return start_minimal_server(port).await;
        }
    };
    
    let http_client = Client::new();
    let router = api::routes::create_router(db_pool, http_client);
    
    let app = Router::new()
        .nest("/api/v1", router)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    
    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await
        .expect("Failed to bind to address");
    
    println!("Server starting on {}", bind_addr);
    
    axum::serve(listener, app).await.expect("Server failed to start");
}

async fn start_minimal_server(port: String) {
    use axum::routing::get;
    
    let app = Router::new()
        .route("/", get(|| async { "Hello from Minimal Rust Server on Render!" }))
        .route("/health", get(|| async { "OK" }))
        .route("/api/v1/health", get(|| async { r#"{"status":"healthy","service":"uniswap_leaderboard_backend_minimal"}"# }))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    
    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await
        .expect("Failed to bind minimal server");
    
    println!("Minimal server starting on {}", bind_addr);
    
    axum::serve(listener, app).await.expect("Minimal server failed to start");
}
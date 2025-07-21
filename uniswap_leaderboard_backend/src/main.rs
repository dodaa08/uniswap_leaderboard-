use axum::{Router, routing::get};
use std::env;

#[tokio::main]
async fn main() {
    println!("=== MINIMAL RUST SERVER FOR RENDER TESTING ===");
    
    // Print all environment variables for debugging
    println!("Environment variables:");
    for (key, value) in env::vars() {
        if key.contains("PORT") || key.contains("RENDER") {
            println!("  {}: {}", key, value);
        }
    }
    
    // Get PORT - this is critical for Render
    let port = env::var("PORT").unwrap_or_else(|_| {
        println!("WARNING: PORT not set, defaulting to 3000");
        "3000".to_string()
    });
    
    println!("Attempting to use PORT: {}", port);
    
    // Create the simplest possible router
    let app = Router::new()
        .route("/", get(|| async { "Hello from Minimal Rust Server on Render!" }))
        .route("/health", get(|| async { "OK" }));
    
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
    
    println!("Server starting on {}", bind_addr);
    println!("Render should detect this port now!");
    
    // Start the server
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }
}
// src/api/sync.rs

use crate::models::{GraphQlResponse, Swap};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use reqwest::Client;
use serde_json::json;
use sqlx::PgPool;

// pair : 0xEdc625B74537eE3a10874f53D170E9c17A906B9c
// zora : 0x1111111111166b7FE7bd91427724B487980aFc69
// usdc : 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913

const THE_GRAPH_URL: &str = "https://gateway.thegraph.com/api/458c04da34a7940de75b87e25a6f9f80/subgraphs/id/HMuAwufqZ1YCRmzL2SfHTVkzZovC9VL2UAKhjvRqKiR1";
// --- UPDATE THIS LINE ---
const POOL_ID: &str = "0xEdc625B74537eE3a10874f53D170E9c17A906B9c"; // This is the ZORA/USDC Pool you confirmed via cURL!
// --- END UPDATE ---

// This is the simplified handler that returns the fetched data.
pub async fn sync_handler(
    State(http_client): State<Client>,
) -> Result<Json<Vec<Swap>>, StatusCode> {
    println!("->> SYNC HANDLER - Fetching historical trades...");

    // The rest of the logic remains the same
    let swaps = fetch_swaps(&http_client)
        .await
        .map_err(|e| {
            eprintln!("    Error fetching swaps: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    println!("    Fetched {} swaps. Returning them as JSON.", swaps.len());
    Ok(Json(swaps))
}

// This function now fetches swaps from a 1-hour window that happened 7 days ago.
async fn fetch_swaps(http_client: &Client) -> Result<Vec<Swap>, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let end_time = now - Duration::days(7); // Calculates the end of the window 7 days ago from *now*
    let start_time = end_time - Duration::hours(1); // Calculates 1 hour before that end time

    let start_timestamp = start_time.timestamp();
    let end_timestamp = end_time.timestamp();

    let query = r#"
        query getSwapsInTimeRange($pool_id: String!, $start_time: BigInt!, $end_time: BigInt!) {
          swaps(first: 100, orderBy: timestamp, orderDirection: asc, where: { pool: $pool_id, timestamp_gte: $start_time, timestamp_lte: $end_time }) {
            timestamp, amountUSD, origin, token0 { symbol }, token1 { symbol }, amount0, amount1
          }
        }
    "#;
    
    let variables = json!({
        "pool_id": POOL_ID, // This will now use the correct ZORA/USDC pool ID
        "start_time": start_timestamp.to_string(),
        "end_time": end_timestamp.to_string()
    });
    
    let request_body = json!({ "query": query, "variables": variables });

    println!("    Fetching swaps between {} and {}", start_time, end_time);
    let response = http_client.post(THE_GRAPH_URL).json(&request_body).send().await?;
    let graph_response: GraphQlResponse = response.json().await?;
    
    Ok(graph_response.data.map_or_else(Vec::new, |d| d.swaps))
}



// src/models.rs

use chrono::{DateTime, Utc}; // Import chrono types
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlResponse {
    pub data: Option<ResponseData>,
    // It's good practice to also include an errors field for GraphQL responses
    pub errors: Option<serde_json::Value>, // Use serde_json::Value to capture any error structure
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub swaps: Vec<Swap>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Swap {
    pub timestamp: String,
    #[serde(rename = "amountUSD")] // <--- FIX: Explicitly rename for "amountUSD" from The Graph
    pub amount_usd: String, // Keep as String as The Graph returns it as such (BigInt/Decimal string)
    pub origin: String, // This is the trader's address
    pub token0: TokenInfo,
    pub token1: TokenInfo,
    pub amount0: String, // Amount of token0 swapped
    pub amount1: String, // Amount of token1 swapped
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub symbol: String,
    // Add other fields from TokenInfo if you query them, e.g.:
    // pub id: String, // Token contract address
    // pub name: String,
    // pub decimals: String, // Decimals are often strings from subgraphs
}

// In src/models.rs
// This struct is for your database interaction, separate from GraphQL response parsing
#[derive(Debug, Serialize, FromRow)]
pub struct Trader {
    pub address: String,
    pub buy_count: i32,
    pub sell_count: i32,
    pub total_volume_usd: rust_decimal::Decimal,
    pub first_trade_at: Option<DateTime<Utc>>, 
    pub last_trade_at: Option<DateTime<Utc>>,
}
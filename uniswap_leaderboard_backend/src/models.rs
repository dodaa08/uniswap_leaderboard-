use chrono::{DateTime, Utc}; // Import chrono types
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlResponse {
    pub data: Option<ResponseData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub swaps: Vec<Swap>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Swap {
    pub id: String,
    pub timestamp: String,
    pub amount_usd: String,
    pub origin: String, // This is the trader's address
    pub token0: TokenInfo,
    pub token1: TokenInfo,
    pub amount0: String,
    pub amount1: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub symbol: String,
}


// In src/models.rs
// In src/models.rs

#[derive(Debug, Serialize, FromRow)]
pub struct Trader {
    pub address: String,
    pub buy_count: i32,
    pub sell_count: i32,
    pub total_volume_usd: rust_decimal::Decimal,
    // Make sure this field name matches the new column name
    pub first_trade_at: Option<DateTime<Utc>>, 
    pub last_trade_at: Option<DateTime<Utc>>,
}

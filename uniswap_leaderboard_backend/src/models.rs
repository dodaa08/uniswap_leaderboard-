use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use sqlx::FromRow;

fn serialize_bigdecimal<S>(value: &sqlx::types::BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlResponse {
    pub data: Option<ResponseData>,
    pub errors: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub swaps: Vec<Swap>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Swap {
    pub timestamp: String,
    #[serde(rename = "amountUSD")]
    pub amount_usd: String,
    pub origin: String,
    pub token0: TokenInfo,
    pub token1: TokenInfo,
    pub amount0: String,
    pub amount1: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub id: String,
    pub symbol: String,
}

#[derive(Debug, FromRow)]
pub struct Trader {
    pub address: String,
    pub buy_count: i32,
    pub sell_count: i32,
    pub total_volume_usd: sqlx::types::BigDecimal,
    pub first_trade_at: Option<DateTime<Utc>>, 
    pub last_trade_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct TraderResponse {
    pub address: String,
    pub buy_count: i32,
    pub sell_count: i32,
    #[serde(serialize_with = "serialize_bigdecimal")]
    pub total_volume_usd: sqlx::types::BigDecimal,
    pub first_trade_at: Option<DateTime<Utc>>, 
    pub last_trade_at: Option<DateTime<Utc>>,
}

impl From<Trader> for TraderResponse {
    fn from(trader: Trader) -> Self {
        TraderResponse {
            address: trader.address,
            buy_count: trader.buy_count,
            sell_count: trader.sell_count,
            total_volume_usd: trader.total_volume_usd,
            first_trade_at: trader.first_trade_at,
            last_trade_at: trader.last_trade_at,
        }
    }
}


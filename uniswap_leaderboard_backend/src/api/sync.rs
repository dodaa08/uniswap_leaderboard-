// src/api/sync.rs

use crate::models::{GraphQlResponse, Swap, Trader, TraderResponse};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc, DateTime};
use reqwest::Client;
use serde_json::json;
use sqlx::PgPool;
use sqlx::types::BigDecimal;
use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::Zero;

const THE_GRAPH_API_KEY: &str = "458c04da34a7940de75b87e25a6f9f80";
const THE_GRAPH_SUBGRAPH_ID: &str = "HMuAwufqZ1YCRmzL2SfHTVkzZovC9VL2UAKhjvRqKiR1";
const TRACKED_TOKEN_CONTRACT_ADDRESS: &str = "0x1111111111166b7FE7bd91427724B487980aFc69";
const TRACKED_POOL_ID: &str = "0xEdc625B74537eE3a10874f53D170E9c17A906B9c";
const GRAPH_PAGE_SIZE: u32 = 1000;

pub async fn sync_handler(
    State((pool, http_client)): State<(PgPool, Client)>,
) -> Result<Json<Vec<TraderResponse>>, StatusCode> {
    let swaps = fetch_swaps(&http_client)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if swaps.is_empty() {
        return Ok(Json(Vec::new()));
    }

    let mut traders_data: HashMap<String, Trader> = HashMap::new();

    for swap in swaps {
        let origin_address = swap.origin.to_lowercase();
        
        let timestamp_i64 = swap.timestamp.parse::<i64>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let swap_time = DateTime::from_timestamp(timestamp_i64, 0).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let amount_usd_decimal: BigDecimal = BigDecimal::from_str(&swap.amount_usd).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let amount0_val: BigDecimal = BigDecimal::from_str(&swap.amount0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let amount1_val: BigDecimal = BigDecimal::from_str(&swap.amount1).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let is_buy_of_tracked_token;
        let is_sell_of_tracked_token;

        if swap.token0.id.eq_ignore_ascii_case(TRACKED_TOKEN_CONTRACT_ADDRESS) {
            if amount0_val < BigDecimal::zero() {
                is_buy_of_tracked_token = true;
                is_sell_of_tracked_token = false;
            } else {
                is_buy_of_tracked_token = false;
                is_sell_of_tracked_token = true;
            }
        } else if swap.token1.id.eq_ignore_ascii_case(TRACKED_TOKEN_CONTRACT_ADDRESS) {
            if amount1_val < BigDecimal::zero() {
                is_buy_of_tracked_token = true;
                is_sell_of_tracked_token = false;
            } else {
                is_buy_of_tracked_token = false;
                is_sell_of_tracked_token = true;
            }
        } else {
            continue;
        }

        let trader = traders_data.entry(origin_address.clone()).or_insert_with(|| Trader {
            address: origin_address,
            buy_count: 0,
            sell_count: 0,
            total_volume_usd: BigDecimal::from(0),
            first_trade_at: None,
            last_trade_at: None,
        });

        if is_buy_of_tracked_token {
            trader.buy_count += 1;
        } else if is_sell_of_tracked_token {
            trader.sell_count += 1;
        }
        trader.total_volume_usd += amount_usd_decimal.abs();

        if trader.first_trade_at.is_none() || swap_time < trader.first_trade_at.unwrap() {
            trader.first_trade_at = Some(swap_time);
        }
        if trader.last_trade_at.is_none() || swap_time > trader.last_trade_at.unwrap() {
            trader.last_trade_at = Some(swap_time);
        }
    }

    for (_address, trader) in traders_data.iter() {
        sqlx::query!(
            r#"
            INSERT INTO traders (address, buy_count, sell_count, total_volume_usd, first_trade_at, last_trade_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (address) DO UPDATE SET
                buy_count = traders.buy_count + EXCLUDED.buy_count,
                sell_count = traders.sell_count + EXCLUDED.sell_count,
                total_volume_usd = traders.total_volume_usd + EXCLUDED.total_volume_usd,
                first_trade_at = LEAST(traders.first_trade_at, EXCLUDED.first_trade_at),
                last_trade_at = GREATEST(traders.last_trade_at, EXCLUDED.last_trade_at)
            "#,
            trader.address,
            trader.buy_count,
            trader.sell_count,
            trader.total_volume_usd,
            trader.first_trade_at,
            trader.last_trade_at,
        )
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(traders_data.into_values().map(|t| t.into()).collect()))
}

async fn fetch_swaps(http_client: &Client) -> Result<Vec<Swap>, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let end_time = now;
    let start_time = now - Duration::hours(24);

    let start_timestamp = start_time.timestamp();
    let end_timestamp = end_time.timestamp();

    let mut all_swaps: Vec<Swap> = Vec::new();
    let mut skip = 0;

    let the_graph_url = format!(
        "https://gateway.thegraph.com/api/{}/subgraphs/id/{}",
        THE_GRAPH_API_KEY, THE_GRAPH_SUBGRAPH_ID
    );

    loop {
        let query = r#"
            query getSwapsInTimeRange($pool_id: String!, $start_time: BigInt!, $end_time: BigInt!, $skip: Int!) {
              swaps(
                first: 1000,
                skip: $skip,
                orderBy: timestamp,
                orderDirection: asc,
                where: { pool: $pool_id, timestamp_gte: $start_time, timestamp_lte: $end_time }
              ) {
                timestamp,
                amountUSD,
                origin,
                token0 { id symbol },
                token1 { id symbol },
                amount0,
                amount1
              }
            }
        "#;

        let variables = json!({
            "pool_id": TRACKED_POOL_ID,
            "start_time": start_timestamp.to_string(),
            "end_time": end_timestamp.to_string(),
            "skip": skip,
        });

        let request_body = json!({ "query": query, "variables": variables });
        
        let response = http_client.post(&the_graph_url).json(&request_body).send().await?;
        let graph_response: GraphQlResponse = response.json().await?;

        if let Some(errors) = graph_response.errors {
            return Err(format!("GraphQL API returned errors: {:?}", errors).into());
        }

        let current_swaps = graph_response.data
            .map_or_else(Vec::new, |d| d.swaps);

        if current_swaps.is_empty() {
            break;
        }

        let current_len = current_swaps.len();
        all_swaps.extend(current_swaps);

        if current_len < (GRAPH_PAGE_SIZE as usize) {
            break;
        }

        skip += GRAPH_PAGE_SIZE;
    }

    Ok(all_swaps)
}

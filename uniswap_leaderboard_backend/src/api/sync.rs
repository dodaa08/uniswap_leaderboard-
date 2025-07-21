// src/api/sync.rs

use crate::models::{GraphQlResponse, Swap};
use axum::{extract::State, http::StatusCode, Json};
use reqwest::Client;
use serde_json::json;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

// src/api/sync.rs

// Use this new, stable, and public URL
const THE_GRAPH_URL: &str = "https://subgraph.satsuma-xyz.net/satsuma/messari/uniswap-v3-base/api";

const POOL_ID: &str = "0x0fa0fb87a0ced71ae1c71bb0a7256433a2c56877";


#[derive(Debug, Default)]
struct TraderStats {
    buys: i32,
    sells: i32,
    total_volume_usd: f64,
}

pub async fn sync_handler(
    State((db_pool, http_client)): State<(PgPool, Client)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("->> SYNC HANDLER - Fetching swaps...");

    let last_timestamp = get_last_timestamp(&db_pool).await.unwrap_or(0);
    println!("    Last timestamp from DB: {}", last_timestamp);

    let swaps = fetch_swaps(&http_client, last_timestamp)
        .await
        .map_err(|e| {
            eprintln!("    Error fetching swaps: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let swaps_count = swaps.len();
    println!("    Fetched {} new swaps.", swaps_count);

    if swaps_count == 0 {
        let response = json!({ "status": "success", "message": "No new swaps to process." });
        return Ok(Json(response));
    }

    let processed_traders = process_and_save_swaps(&db_pool, swaps).await.map_err(|e| {
        eprintln!("    Error processing and saving swaps: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = json!({
        "status": "success",
        "swaps_fetched": swaps_count,
        "traders_updated": processed_traders,
    });

    Ok(Json(response))
}

async fn process_and_save_swaps(
    db_pool: &PgPool,
    swaps: Vec<Swap>,
) -> Result<usize, sqlx::Error> {
    let mut trader_stats: HashMap<String, TraderStats> = HashMap::new();

    for swap in swaps {
        let stats = trader_stats.entry(swap.origin).or_default();
        let volume_usd: f64 = swap.amount_usd.parse().unwrap_or(0.0);
        let amount1: f64 = swap.amount1.parse().unwrap_or(0.0);

        if amount1 > 0.0 {
            stats.buys += 1;
        } else {
            stats.sells += 1;
        }
        stats.total_volume_usd += volume_usd;
    }

    for (address, stats) in &trader_stats {
        let volume_decimal = rust_decimal::Decimal::try_from(stats.total_volume_usd).unwrap();
        sqlx::query(
            r#"
            INSERT INTO traders (address, buy_count, sell_count, total_volume_usd, first_trade_at, last_trade_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            ON CONFLICT (address) DO UPDATE
            SET
                buy_count = traders.buy_count + $2,
                sell_count = traders.sell_count + $3,
                total_volume_usd = traders.total_volume_usd + $4,
                last_trade_at = NOW(),
                updated_at = NOW();
            "#
        )
        .bind(address)
        .bind(stats.buys)
        .bind(stats.sells)
        .bind(volume_decimal)
        .execute(db_pool)
        .await?;
    }

    Ok(trader_stats.len())
}

async fn fetch_swaps(
    http_client: &Client,
    last_timestamp: i64,
) -> Result<Vec<Swap>, Box<dyn std::error::Error>> {
    // Use timestamp from DB or 0 if empty
    let timestamp_to_use = if last_timestamp == 0 {
        "0"
    } else {
        &last_timestamp.to_string()
    };
    
    let query = format!(r#"
        query getSwapsForPool {{
          swaps(
            first: 100
            orderBy: timestamp
            orderDirection: asc
            where: {{
              pool: "0x0fa0fb87a0ced71ae1c71bb0a7256433a2c56877",
              timestamp_gt: "{}"
            }}
          ) {{
            id
            timestamp
            token0 {{
              symbol
            }}
            token1 {{
              symbol
            }}
            amount0
            amount1
            amountUSD
            origin
          }}
        }}
    "#, timestamp_to_use);
    
    let request_body = json!({ "query": query });
    
    println!("    Making GraphQL request to: {}", THE_GRAPH_URL);
    println!("    Query: {}", query);
    
    let response = http_client.post(THE_GRAPH_URL).json(&request_body).send().await?;
    let response_text = response.text().await?;
    
    println!("    GraphQL response: {}", response_text);
    
    let graph_response: GraphQlResponse = serde_json::from_str(&response_text)?;
    Ok(graph_response.data.map_or_else(Vec::new, |d| d.swaps))
}

async fn get_last_timestamp(db_pool: &PgPool) -> Result<i64, sqlx::Error> {
    println!("    Querying database for last timestamp...");
    let result = sqlx::query("SELECT EXTRACT(EPOCH FROM MAX(last_trade_at)) as max FROM traders")
        .fetch_one(db_pool)
        .await?;
    let timestamp = result.try_get::<Option<f64>, _>("max")?.map_or(0, |v| v as i64);
    println!("    Database returned timestamp: {}", timestamp);
    Ok(timestamp)
}
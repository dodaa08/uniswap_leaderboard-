use crate::models::{Trader, TraderResponse};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_page_size")]
    page_size: u32,
}

fn default_page() -> u32 { 1 }
fn default_page_size() -> u32 { 20 }

pub async fn leaderboard_handler(
    State(db_pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<TraderResponse>>, StatusCode> {
    let limit = pagination.page_size;
    let offset = (pagination.page - 1) * pagination.page_size;

    let traders = sqlx::query_as::<_, Trader>(
        r#"
        SELECT address, buy_count, sell_count, total_volume_usd, first_trade_at, last_trade_at
        FROM traders
        ORDER BY total_volume_usd DESC
        LIMIT $1 OFFSET $2
        "#
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let trader_responses: Vec<TraderResponse> = traders.into_iter().map(|t| t.into()).collect();
    Ok(Json(trader_responses))
}

pub async fn trader_handler(
    State(db_pool): State<PgPool>,
    Path(address): Path<String>,
) -> Result<Json<TraderResponse>, StatusCode> {
    let trader = sqlx::query_as::<_, Trader>(
        r#"
        SELECT address, buy_count, sell_count, total_volume_usd, first_trade_at, last_trade_at
        FROM traders
        WHERE address = $1
        "#
    )
    .bind(address)
    .fetch_optional(&db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(trader.into()))
}
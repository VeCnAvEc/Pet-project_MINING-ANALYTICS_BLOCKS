use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BlockModel {
    pub id: u32,
    pub hash: String,
    pub height: i64,
    pub timestamp: DateTime<Utc>,
    pub transaction_count: i32,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i32,
    pub txid: String,
    pub block_hash: Option<String>,
    pub fee: i64,
    pub size: i32,
    pub created_at: DateTime<Utc>
}

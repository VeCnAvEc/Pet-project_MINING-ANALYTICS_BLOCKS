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
    pub full_reward: u64,
    pub fee: u64,
    pub size: i32,
    pub is_coinbase: bool,
    pub main_reward: Option<i64>,
    pub miner_address: Option<String>,
    pub guessed_miner: Option<String>,
    pub created_at: DateTime<Utc>
}
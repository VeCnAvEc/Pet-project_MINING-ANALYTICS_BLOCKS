use std::sync::Arc;
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres, Row};

use anyhow::Result;

use chrono::Utc;

use log::info;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::time::{timeout, Duration as TokioDuration};

use crate::infrastructure::queue::queue_service::BlockAnalyticsMessage;

pub struct Database {
    pool: Arc<Pool<Postgres>>,
    pub sender: mpsc::Sender<BlockAnalyticsMessage>,
    // pub receiver: Receiver<BlockAnalyticsMessage>
}

impl Database {
    pub async fn new(database_url: &str) -> Result<(Arc<Self>, Receiver<BlockAnalyticsMessage>)> {
        info!("Connected with PostgreSQL");
        let (sender, receiver) = mpsc::channel::<BlockAnalyticsMessage>(1000);

        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url)
            .await?;

        info!(
            "save_block_and_coinbase(): pool before begin: size={}, idle={}, acquired={}",
            pool.size(),
            pool.num_idle(),
            (pool.size() as i32) - (pool.num_idle() as i32)
        );

        info!("Successfully connected to PostgreSQL");

        let database = Arc::new(Self { pool: Arc::new(pool), sender });

        Ok((database, receiver))
    }

    pub fn pool(&self) -> Arc<PgPool> { Arc::clone(&self.pool) }

    // pub async fn run_migration(&self) -> Result<()> {
    //     info!("Running migrations");
    //     sqlx::migrate!("./migrations").run(&self.pool).await?;
    //     info!("Migration was completed successfully.");
    //     Ok(())
    // }

    pub async fn save_block_and_coinbase(
        pool: Arc<PgPool>,
        message: &BlockAnalyticsMessage,
    ) -> Result<(i32, i32)> {
        let mut tx = pool.begin().await?;

        let ts = chrono::DateTime::from_timestamp(message.timestamp as i64, 0)
            .ok_or_else(|| anyhow::anyhow!("bad unix timestamp: {}", message.timestamp))?;

        let upsert_block_sql = r#"
            INSERT INTO blocks (hash, height, "timestamp", transactions_count, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (hash) DO NOTHING
            RETURNING id
        "#;

        let block_row = sqlx::query(upsert_block_sql)
            .bind(&message.block_hash)
            .bind(message.height as i64)
            .bind(ts)
            .bind(message.transactions_count as i64)
            .bind(Utc::now())
            .fetch_one(&mut *tx)
            .await?;
        let block_id = block_row.get::<i32, _>("id");
        info!("Block upserted {} (hash={}) with id={}", message.height, message.block_hash, block_id);

        let coinbase = &message.coinbase_info;
        let txid = format!("coinbase_{}", message.block_hash);

        let upsert_tx_sql = r#"
            INSERT INTO transactions (
                txid, block_hash, fee, size, is_coinbase,
                main_reward, miner_address, full_reward, guessed_miner, created_at
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            ON CONFLICT (txid) DO NOTHING
            RETURNING id
        "#;

        let tx_row = sqlx::query(upsert_tx_sql)
            .bind(&txid)
            .bind(&message.block_hash)
            .bind(coinbase.fee)
            .bind(message.size as i64)
            .bind(true)
            .bind(coinbase.main_reward)
            .bind(coinbase.miner_address.clone())
            .bind(coinbase.full_reward)
            .bind(coinbase.guessed_miner.clone())
            .bind(Utc::now())
            .fetch_one(&mut *tx)
            .await?;


        let transaction_id = tx_row.get::<i32, _>("id");
        info!(
            "Coinbase upserted for block hash={} with txid={} id={}",
            message.block_hash, txid, transaction_id
        );

        timeout(TokioDuration::from_secs(3), tx.commit()).await??;

        Ok((block_id, transaction_id))
    }

    pub async fn queue_messages_reader(mut receiver: Receiver<BlockAnalyticsMessage>, pool: Arc<PgPool>) {
        while let Some(message) = receiver.recv().await {
            let pool = Arc::clone(&pool);
            let _ = Database::save_block_and_coinbase(pool, &message).await;
        }
    }
}
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use anyhow::Result;
use chrono::{DateTime, Utc};
use log::info;
use crate::infrastructure::queue::queue_service::BlockAnalyticsMessage;

#[derive(sqlx::FromRow)]
struct BlockId {
    id: i32,
}

pub struct Database {
    pool: PgPool
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Connected with PostgreSQL");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        info!("Successfully connected to PostgreSQL");

        Ok( Self { pool } )
    }

    pub fn pool(&self) -> &PgPool { &self.pool }

    pub async fn run_migration(&self) -> Result<()> {
        info!("Running migrations");
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        info!("Migration was completed successfully.");
        Ok(())
    }

    pub async fn save_block(&self, message: &BlockAnalyticsMessage) -> Result<i32> {
        let block_id = sqlx::query(
            r#"
            INSERT INTO blocks (hash, height, timestamp, transactions_count, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
        )
        .bind(&message.block_hash)
        .bind(message.height as i64)
        .bind(DateTime::from_timestamp(message.timestamp as i64, 0).unwrap())
        .bind(0)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?
        .get::<i32, _>("id");

        info!("Block saved {} to DB with ID: {}", message.height, block_id);

        Ok(block_id)
    }
}
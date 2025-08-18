use std::sync::Arc;
use anyhow::Result;
use log::info;
use rabbitmq_stream_client::types::OffsetSpecification;
use crate::config::config::Config;
use crate::infrastructure::db::postgres::Database;
use crate::infrastructure::queue::queue_service::QueueService;

pub struct MessageIngestionService {
    rabbit_queue_service: Option<Arc<QueueService>>,
    db: Arc<Database>,
    config: Arc<Config>,
}

impl MessageIngestionService {
    pub fn new(config: Arc<Config>, queue_service: Option<Arc<QueueService>>, db: Arc<Database>) -> Self {
        Self {
            rabbit_queue_service: queue_service,
            db,
            config,
        }
    }

    pub async fn start_monitoring_rabbit_messages(&mut self) -> Result<()> {
        if self.rabbit_queue_service.is_none() {
            return Err(anyhow::anyhow!("Error RabbitMQ: Couldn't to connect with RabbitMQ"));
        }

        // Возможно чуть позже подумаю, как быть, если вдруг rabbitmq не работает, вдруг решу записывать на прямую в db.
        if let Some(queue_service) = &self.rabbit_queue_service {
            let rabbitmq_client = Arc::clone(&queue_service.rabbitmq_client);
            let db = Arc::clone(&self.db);

            let environment = rabbitmq_client.get_environment();

            let consumer_mining_analytics = environment
                .consumer()
                .name("reader-for-block-analytics")
                .offset(OffsetSpecification::Next)
                .build("mining-analytics")
                .await?;

            QueueService::read_messages_from_rabbitmq_mining_analytics(consumer_mining_analytics, db).await;
        }

        Ok(())
    }

    pub fn get_db(&self) -> Arc<Database> { Arc::clone(&self.db) }
}
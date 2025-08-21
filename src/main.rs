mod config;
mod infrastructure;
mod application;
mod domain;
mod utils;
mod scheduler;
mod logs;

use std::sync::Arc;

use log::error;

use reqwest::Client;

use tracing::info;

use crate::config::config::Config;
use crate::infrastructure::db::postgres::Database;
use crate::infrastructure::queue::queue_service::QueueService;
use crate::infrastructure::queue::stream_rabbitmq::RabbitMQClient;
use crate::logs::init_logs::init_tracing;
use crate::scheduler::SchedulerManager;

#[tokio::main]
async fn main() {
    init_tracing();
    let reqwest_client = Arc::new(Client::new());

    let config = Arc::new(Config::new());
    info!("Config: {:?}", config);

    let rabbit_mq_client = RabbitMQClient::new(config.get_rabbitmq_config())
        .await
        .inspect_err(|e| error!("Error creating RabbitMq client: {}", e))
        .ok();

    let db = Database::new(config.get_database_url()).await.ok();

    let queue_service = match rabbit_mq_client {
        None => {
            info!("RabbitMQ client creation failed, continuing without queue service.");
            None
        }
        Some(rabbitmq) => {
            info!("RabbitMQ client created successfully.");
            Some(Arc::new(QueueService::new(Arc::new(rabbitmq))))
        }
    };


    let client_for_scheduler = Arc::clone(&reqwest_client);
    let config_for_scheduler = Arc::clone(&config);

    let mut scheduler = SchedulerManager::new(config_for_scheduler, client_for_scheduler);

    scheduler.launch_all_tasks(queue_service, db).await;
    scheduler.wait_for_all_tasks().await;
}
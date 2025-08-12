mod config;
mod infrastructure;
mod application;
mod domain;
mod utils;
mod scheduler;

use std::sync::Arc;
use log::error;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::config::Config;
use crate::infrastructure::queue::queue_service::QueueService;
use crate::infrastructure::queue::stream_rabbitmq::RabbitMQClient;
use crate::scheduler::SchedulerManager;

#[tokio::main]
async fn main() {
    let subscribe = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();

    subscribe.init();

    let reqwest_client = Arc::new(Client::new());
    let config = Arc::new(Config::new());
    info!("Config: {:?}", config);
    let rabbit_mq_client = RabbitMQClient::new(config.get_rabbitmq_config())
        .await
        .inspect_err(|e| error!("Error creating RabbitMq client: {}", e))
        .ok();

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

    scheduler.launch_all_tasks(queue_service).await;
    scheduler.wait_for_all_tasks().await;
}
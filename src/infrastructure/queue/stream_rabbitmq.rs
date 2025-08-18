use std::sync::Arc;

use anyhow::Result;

use log::{error, info};

use tokio::sync::Mutex;

use rabbitmq_stream_client::{Dedup, Environment, Producer};
use rabbitmq_stream_client::types::{ByteCapacity, Message, OffsetSpecification, ResponseCode};
use rabbitmq_stream_client::error::{ConsumerCreateError, StreamCreateError};

use crate::config::config::RabbitMqConfig;

pub struct RabbitMQClient {
    environment: Arc<Environment>,
    block_analytics_producer: Arc<Mutex<Producer<Dedup>>>,
    stream_name: String,
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>
}

impl RabbitMQClient {
    pub async fn new(config: &RabbitMqConfig) -> Result<Self> {
        let block_analytics_stream = "mining-analytics";
        let mining_notifications_stream = "mining-notifications";

        let mining_analytics_producer_name = "mining-analytics-producer";
        // let mining_notifications_producer_name = "mining-notifications-producer";

        let environment = Arc::new(
            Environment::builder()
                .host(config.get_host())
                .port(config.get_port())
                .username(config.get_username().unwrap_or(&"guest".to_string()))
                .password(config.get_password().unwrap_or(&"guest".to_string()))
                .build()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create RabbitMQ environment: {}", e))
                ?
        );

        // Сначала создаем стримы
        RabbitMQClient::create_stream(&environment, block_analytics_stream).await;
        RabbitMQClient::create_stream(&environment, mining_notifications_stream).await;

        // Потом создаем producer
        let analytics_block_producer = Arc::new(Mutex::new(environment
            .producer()
            .name(mining_analytics_producer_name)
            .build(block_analytics_stream)
            .await?));

        info!("RabbitMq client initialized successfully");

        Ok(Self {
            environment: Arc::clone(&environment),
            block_analytics_producer: analytics_block_producer,
            stream_name: config.get_stream_name().to_string(),
            host: config.get_host().to_string(),
            port: config.get_port(),
            password: config.get_password().cloned(),
            username: config.get_password().cloned()
        })
    }

    pub async fn send_to_stream<T: serde::Serialize>(&self, data: &Vec<T>) -> Result<()> {
        let messages: Vec<Message> = data.into_iter()
            .map(|data| {
                let json_bytes = serde_json::to_vec(&data).unwrap();
                Message::builder().body(json_bytes).build()
            })
            .collect();

        self.block_analytics_producer.lock().await.batch_send(messages, |res| async move {
            match res {
                Ok(confirmation_status) => {
                    let status = confirmation_status.status();
                    match status {
                        ResponseCode::Ok => info!("Messages send successfully!"),
                        other_response => {
                            error!("Couldn't send messages, response with status: {:?}", other_response);
                        }
                    }
                },
                Err(e) => error!("Batch send with error: {:?}", e),
            }
        }).await?;

        Ok(())
    }


    pub async fn send_batch_analytics_messages<T: serde::Serialize>(&self, data: &Vec<T>) -> Result<()> {
        self.send_to_stream(data).await
    }

    pub async fn send_batch_notification_messages<T: serde::Serialize>(&self, data: &Vec<T>) -> Result<()> {
        self.send_to_stream(data).await
    }

    pub fn get_environment(&self) -> Arc<Environment> {
        Arc::clone(&self.environment)
    }

    pub fn get_stream_name(&self) -> &str {
        &self.stream_name
    }

    pub async fn create_stream(environment: &Environment, consumer_name: &str) {
        let create_response = environment
            .stream_creator()
            .max_length(ByteCapacity::GB(5))
            .create(consumer_name)
            .await;

        if let Err(e) = create_response {
            if let StreamCreateError::Create { stream, status } = e {
                match status {
                    // we can ignore this error because the stream already exists
                    ResponseCode::StreamAlreadyExists => {}
                    err => {
                        println!("Error creating stream: {:?} {:?}", stream, err);
                    }
                }
            }
        }
    }
}
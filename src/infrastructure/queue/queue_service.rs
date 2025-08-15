use std::sync::Arc;

use log::{error, info};

use serde::{Deserialize, Serialize};

use anyhow::Result;

use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::domain::block::Block;
use crate::domain::transaction::Transaction;
use crate::infrastructure::queue::stream_rabbitmq::RabbitMQClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockAnalyticsMessage {
    pub height: u32,
    pub block_hash: String,
    pub timestamp: u64,
    pub size: u32,
    pub merkle_root: String,
    pub difficulty: f64,
    pub coinbase_info: CoinbaseInfo
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinbaseInfo {
    pub main_reward: Option<u64>,
    pub miner_address: Option<String>,
    pub full_reward: u64,
    pub guessed_miner: String,
    pub rewards_and_addresses: Vec<(u64, String)>
}

pub struct QueueService {
    pub(crate) rabbitmq_client: Arc<RabbitMQClient>,
    pub sender: mpsc::Sender<BlockAnalyticsMessage>,
    worker_handle: Option<JoinHandle<()>>
}

impl QueueService {
    pub fn new(rabbitmq_client: Arc<RabbitMQClient>) -> Self {
        let (sender, receiver) = mpsc::channel::<BlockAnalyticsMessage>(1000);

        let worker_rabbitmq = Arc::clone(&rabbitmq_client);
        let worker_handle = tokio::spawn(async move {
            Self::queue_worker(receiver, worker_rabbitmq).await;
        });

        Self {
            rabbitmq_client,
            sender,
            worker_handle: Some(worker_handle)
        }
    }

    pub async fn send_block_analytics(&self, block: &Block, coinbase: &Transaction, guessed_miner: String) -> Result<()> {
        let analytics_message = BlockAnalyticsMessage {
            height: block.get_height(),
            block_hash: block.get_id().to_string(),
            timestamp: block.get_timestamp(),
            size: block.get_size(),
            merkle_root: block.get_merkle_root().to_string(),
            difficulty: block.get_difficulty(),
            coinbase_info: CoinbaseInfo {
                main_reward: coinbase.get_main_reward_value(),
                miner_address: coinbase.get_main_reward_address().and_then(|addr| addr.clone()),
                full_reward: coinbase.get_full_reward_value(),
                guessed_miner,
                rewards_and_addresses: coinbase.get_rewards_value_and_address(),
            },
        };

        match self.sender.send(analytics_message).await {
            Ok(_) => {
                info!("Block analytics queue for block {}", block.get_height());
                Ok(())
            }
            Err(e) => {
                error!("Failed to queue block analytics: {:?}", e);
                Err(anyhow::anyhow!("Channel send error: {}", e))
            }
        }
    }

    pub async fn send_notification(&self, message: &str) -> Result<()> {
        #[derive(Serialize)]
        struct Notification {
            message: String,
            timestamp: u64,
        }

        let notification_message = Notification {
            message: message.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                ?.as_secs()
        };

        // self.rabbitmq_client.send_batch_notification_messages(&notification_message).await?;

        Ok(())
    }

    pub async fn queue_worker(mut reciver: mpsc::Receiver<BlockAnalyticsMessage>, rabbitmq_client: Arc<RabbitMQClient>) {
        info!("Queue worker started!");
        let batch_size: usize = 10;

        let mut batch_analytics_messages = Vec::new();

        while let Some(message) = reciver.recv().await {
            let height = message.height;
            batch_analytics_messages.push(message);

            if batch_analytics_messages.len() >= batch_size {
                match rabbitmq_client.send_batch_analytics_messages(&batch_analytics_messages).await {
                    Ok(_) => {
                        info!("Successfully analytics message to RabbitMQ for block {}", height);
                        batch_analytics_messages.clear();
                    }
                    Err(err) => {
                        error!("Failed to send analytics message to RabbitMQ for block {}: {:?}", height, err);
                        info!("Messages from batch_analytics_messages: {:?} deleted.", batch_analytics_messages);
                        batch_analytics_messages.clear();
                    }
                }
            }
        }

        info!("Queue worker stopped");
    }

    // pub async fn shutdown(&mut self) {
    //     drop(self.sender);
    //
    //     if let Some(handle) = self.worker_handle.take() {
    //         if let Err(e) = handle.await {
    //             error!("Error waiting for queue worker to shutdown: {:?}", e);
    //         }
    //     }
    // }
}
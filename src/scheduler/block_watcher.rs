use std::sync::Arc;
use std::time::Duration;
use bitcoin::ScriptBuf;
use log::{error, info, warn};
use reqwest::Client;
use crate::config::config::Config;
use crate::domain::block::Block;
use crate::domain::transaction::Transaction;
use crate::infrastructure::collector::mempool::{fetch_get_coinbase, fetch_get_coinbase_tx_id, fetch_latest_blocks};
use crate::infrastructure::queue::queue_service::QueueService;
use crate::utils::script_sig::ParsedScriptSig;

pub struct BlockWatcher {
    client: Arc<Client>,
    config: Arc<Config>,
    rabbitmq_queue_service: Option<Arc<QueueService>>
}

impl BlockWatcher {
    pub fn new(client: Arc<Client>, config: Arc<Config>, queue_service: Option<Arc<QueueService>>) -> Self {
        Self {
            client,
            config,
            rabbitmq_queue_service: queue_service
        }
    }

    pub async fn start_monitoring_new_blocks(&mut self) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.get_interval_analytic_blocks()));

        loop {
            let client_for_blocks = Arc::clone(&self.client);
            let blocks = fetch_latest_blocks(client_for_blocks, self.config.get_api_url().to_string()).await?;

            self.process_blocks(blocks).await;
            interval.tick().await;
        }
    }

    async fn process_blocks(&self, blocks: Vec<Block>) {
        for block in blocks.iter() {
            if let Err(e) = self.process_block_info(block).await {
                error!("Processing block error: {:?}", e);
            }
        }
    }

    async fn process_block_info(&self, block: &Block) -> anyhow::Result<()> {
        let block_hash = block.get_id();

        let client = Arc::clone(&self.client);
        let url = self.config.get_api_url();

        let coinbase_txid = fetch_get_coinbase_tx_id(client, url.to_string(), block_hash.to_string())
            .await
            .map_err(|e| anyhow::anyhow!("Get coinbase tx id error: {}", e))?;

        let client_for_coinbase = Arc::clone(&self.client);
        let coinbase = fetch_get_coinbase(client_for_coinbase, url.to_string(), coinbase_txid)
            .await
            .map_err(|e| anyhow::anyhow!("Get coinbase error: {}", e))?;

        let height = block.get_height();
        let timestamp = block.get_timestamp();
        let size = block.get_size();
        let merkle_root = block.get_merkle_root();
        let difficulty = block.get_difficulty();

        info!("------------  Block information  ------------");
        info!("----   Height: {}   ----", height);
        info!("----   Timestamp: {}   ----", timestamp);
        info!("----   Size: {}   ----", size);
        info!("----   merkle_root: {}   ----", merkle_root);
        info!("----   difficulty: {}   ----", difficulty);
        self.report_coinbase_details(block, &coinbase).await;
        info!("------------  Block information closed  ------------");

        Ok(())
    }

    async fn report_coinbase_details(&self, block: &Block, coinbase: &Transaction) {
        let script_sig = coinbase.get_vin_scriptsig();

        let bytes = hex::decode(script_sig).expect("Invalid hex");
        let script = ScriptBuf::from_bytes(bytes);

        let parsed_script_opt = ParsedScriptSig::from(&script);

        let parsed_script = match parsed_script_opt {
            None => {
                error!("Error parsed scriptSig.");
                return
            },
            Some(parsed_script) => {
                parsed_script
            }
        };

        let main_reward = coinbase.get_main_reward_value();
        let address_miner = coinbase.get_main_reward_address();
        let full_reward = coinbase.get_full_reward_value();
        let rewards_and_addresses = coinbase.get_rewards_value_and_address();
        let guessed_miner = parsed_script.guessed_miner;

        info!("------  Coinbase information  ------");
        info!("--  Main reward: {:?}  --", main_reward);
        info!("--  Miner address: {:?}  --", address_miner);
        info!("--  Full reward: {}  --", full_reward);
        info!("--  Rewards and addresses: {:?}  --", rewards_and_addresses);
        info!("--  Guessed miner: {}  --", guessed_miner);
        info!("------  Closed Coinbase Information  ------");


        if let Some(queue_service) = &self.rabbitmq_queue_service {
            if let Err(e) = queue_service.send_block_analytics(block, coinbase, guessed_miner).await {
                error!("Error sending block analytics: {:?}", e);
            } else {
                info!("Block analytics sent successfully for block {}", block.get_height());
            }
        } else {
            warn!("Queue service not available, skipping analytics sending");
        }
    }
}
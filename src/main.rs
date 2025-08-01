mod config;
mod infrastructure;
mod application;
mod domain;
mod utils;
mod scheduler;

use std::sync::Arc;
use bitcoin::ScriptBuf;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::config::Config;
use crate::infrastructure::collector::mempool::{fetch_get_coinbase, fetch_get_coinbase_tx_id, fetch_latest_blocks};
use crate::infrastructure::collector::namespace::{build_url, NameSpaceApi};
use crate::utils::script_sig::ParsedScriptSig;

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
    let config = Config::new();
    info!("Config: {:?}", config);

    let url = config.get_api_url();

    let client_blocks = Arc::clone(&reqwest_client);
    let client_coinbase_txid = Arc::clone(&reqwest_client);
    let client_coinbase_tx = Arc::clone(&reqwest_client);

    let blocks = fetch_latest_blocks(client_blocks, url.to_string()).await.unwrap();
    let first_hash = blocks[0].get_id();

    let coinbase_txid = fetch_get_coinbase_tx_id(client_coinbase_txid, url.to_string(), first_hash).await.unwrap();
    let coinbase_tx = fetch_get_coinbase(client_coinbase_tx, url.to_string(), coinbase_txid).await.unwrap();

    // info!("Coinbase tx: {:#?}", coinbase_tx);

    // let pubkey_address = coinbase_tx.get_vout_scriptpubkey_address();
    // let value = coinbase_tx.get_vout_value();
    let script_sig = coinbase_tx.get_vin_scriptsig();

    let bytes = hex::decode(script_sig).expect("Invalid hex");
    let script = ScriptBuf::from_bytes(bytes);

    let parsed_script_sig = ParsedScriptSig::from(&script).unwrap();
    let guessed_miner = &parsed_script_sig.guessed_miner;
    let height = parsed_script_sig.block_height;
    let block_time = coinbase_tx.get_status().get_block_time();
    
    let main_reward_value = coinbase_tx.get_main_reward_value();
    let main_reward_address = coinbase_tx.get_main_reward_address();
    let full_reward = coinbase_tx.get_full_reward_value();
    let rewards_and_addresses = coinbase_tx.get_rewards_value_and_address();

    info!("Full reward: {}", full_reward);
    info!("Main reward: {:?}", main_reward_value);
    info!("Main reward address: {:?}", main_reward_address);
    info!("All rewards and addresses: {:?}", rewards_and_addresses);
    info!("Guessed miner: {}", guessed_miner);
    info!("Block height: {}", height);
    info!("Block time: {}", block_time);
}
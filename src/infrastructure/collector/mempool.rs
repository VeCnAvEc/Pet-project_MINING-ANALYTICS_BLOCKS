use std::sync::Arc;
use anyhow::anyhow;
use reqwest::Client;
use serde_json::from_str;
use tracing::info;
use crate::domain::block::Block;
use crate::domain::transaction::Transaction;
use crate::infrastructure::collector::namespace::NameSpaceApi;

pub async fn fetch_latest_blocks(client: Arc<Client>, url: String) -> anyhow::Result<Vec<Block>> {
    let ns = NameSpaceApi::Blocks(None).get_uri_by_ns();
    let url = format!("{url}{ns}");

    let response = client.get(url).send().await?;
    let body = response.text().await?;

    let blocks_result: serde_json::Result<Vec<Block>> = from_str(&body);

    let blocks = if blocks_result.is_ok() {
        blocks_result?
    } else {
        vec![]
    };

    Ok(blocks)
}

pub async fn fetch_get_coinbase_tx_id(client: Arc<Client>, url: String, hash: String) -> anyhow::Result<String> {
    let ns = NameSpaceApi::BlockTxids(hash).get_uri_by_ns();
    let url = format!("{url}{ns}");

    let response = client.get(url).send().await?;
    let body = response.text().await?;

    let txids: Vec<String> = from_str(&body)?;

    let coinbase_txid = txids[0].clone();

    Ok(coinbase_txid)
}

pub async fn fetch_get_coinbase(client: Arc<Client>, url: String, coinbase_txid: String) -> anyhow::Result<Transaction> {
    let ns = NameSpaceApi::TxById(coinbase_txid).get_uri_by_ns();
    let url = format!("{url}{ns}");

    let response = client.get(url).send().await?;
    let body = response.text().await?;

    let coinbase_tx: Transaction = from_str(&body)?;
    let vin = coinbase_tx.get_vin_by_id(0).unwrap();
    let is_coinbase = vin.is_coinbase;
    if is_coinbase {
        Ok(coinbase_tx)
    } else {
        Err(anyhow!("it isn't coinbase"))
    }
}
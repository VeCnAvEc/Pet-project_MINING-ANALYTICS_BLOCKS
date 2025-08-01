use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    id: String,
    height: u64,
    version: u64,
    timestamp: u64,
    tx_count: u64,
    size: u64,
    weight: u64,
    merkle_root: String,
    previousblockhash: String,
    mediantime: u64,
    nonce: u64,
    bits: u64,
    difficulty: f64,
}

impl Block {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
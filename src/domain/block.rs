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

    pub fn get_height(&self) -> u32 {
        self.height as u32
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_merkle_root(&self) -> &str {
        &self.merkle_root
    }

    pub fn get_difficulty(&self) -> f64 {
        self.difficulty
    }

    pub fn get_tx_count(&self) -> u64 {
        self.tx_count
    }
}
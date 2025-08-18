use serde::{Deserialize, Serialize};
use crate::utils::block_reward::BlockRewardCalculator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorInputs {
    txid: String,
    vout: u64,
    prevout: Option<String>,
    scriptsig: String,
    scriptsig_asm: String,
    witness: Vec<String>,
    pub is_coinbase: bool,
    sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorOutputs {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: Option<String>,
    value: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    confirmed: bool,
    block_height: i64,
    block_hash: String,
    block_time: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    txid: String,
    version: u64,
    locktime: u64,
    vin: Vec<VectorInputs>,
    vout: Vec<VectorOutputs>,
    size: u32,
    weight: u32,
    sigops: u32,
    status: Status
}

impl Transaction {
    pub fn get_main_reward_vout(&self) -> Option<&VectorOutputs> {
        self.vout.iter()
            .filter(|vout| vout.value > 0) // Исключаем OP_RETURN (value = 0)
            .max_by_key(|vout| vout.value)
    }

    pub fn get_main_reward_address(&self) -> Option<&Option<String>> {
        self.get_main_reward_vout()
            .map(|vout| vout.get_scriptpubkey_address())
    }

    pub fn get_main_reward_value(&self) -> Option<i64> {
        self.get_main_reward_vout()
            .map(|vout| vout.get_value())
    }

    pub fn get_full_reward_value(&self) -> i64 {
        self.vout.iter()
            .fold(0, |acc, vout| acc + vout.value)
    }

    pub fn get_rewards_value_and_address(&self) -> Vec<(i64, String)> {
        self.vout.iter()
            .filter(|vout| vout.value > 0) // Исключаем OP_RETURN (value = 0)
            .filter_map(|vout| {
                // Получаем адрес, если он есть
                vout.scriptpubkey_address.as_ref()
                    .map(|address| (vout.value, address.clone()))
            })
            .collect()
    }

    pub fn get_vouts(&self) -> &Vec<VectorOutputs> {
        &self.vout
    }

    pub fn get_vin_scriptsig(&self) -> &str {
        let vector_input = &self.vin[0];
        vector_input.get_vin_scriptsig()
    }

    pub fn get_vin_by_id(&self, id: usize) -> Option<&VectorInputs> {
        self.vin.get(id)
    }

    pub fn get_each_vin(&self) -> &Vec<VectorInputs> { &self.vin }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn calculate_fee(&self) -> Option<i64> {
        if self.vin.is_empty() || !self.vin[0].is_coinbase {
            return None;
        }

        let block_height = self.status.block_height;
        // Получаем текущее вознагрождение по высоте блока
        let current_block_reward = BlockRewardCalculator::calculate_block_reward(block_height);
        // Получаем полное вознагрождение за блок
        let total_output_value = self.get_full_reward_value();

        if total_output_value > current_block_reward {
            // Высчитываем комиссию майнеров, для майнера который нашёл блок.
            Some(total_output_value - current_block_reward)
        } else {
            Some(0)
        }
    }
}

impl VectorOutputs {
    pub fn get_scriptpubkey_address(&self) -> &Option<String> {
        &self.scriptpubkey_address
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

impl VectorInputs {
    pub fn get_vin_scriptsig(&self) -> &str {
        &self.scriptsig
    }
}

impl Status {
    pub fn get_block_time(&self) -> u64 {
        self.block_time
    }
}
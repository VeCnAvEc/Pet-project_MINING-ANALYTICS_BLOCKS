use serde::{Deserialize, Serialize};

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
    value: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    confirmed: bool,
    block_height: u64,
    block_hash: String,
    block_time: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    txid: String,
    version: u8,
    locktime: u16,
    vin: Vec<VectorInputs>,
    vout: Vec<VectorOutputs>,
    size: u32,
    weight: u32,
    sigops: u32,
    status: Status
}

impl Transaction {
    // Новые методы для поиска основного выхода с максимальной наградой
    pub fn get_main_reward_vout(&self) -> Option<&VectorOutputs> {
        self.vout.iter()
            .filter(|vout| vout.value > 0) // Исключаем OP_RETURN (value = 0)
            .max_by_key(|vout| vout.value)
    }

    pub fn get_main_reward_address(&self) -> Option<&Option<String>> {
        self.get_main_reward_vout()
            .map(|vout| vout.get_scriptpubkey_address())
    }

    pub fn get_main_reward_value(&self) -> Option<u64> {
        self.get_main_reward_vout()
            .map(|vout| vout.get_value())
    }

    pub fn get_full_reward_value(&self) -> u64 {
        self.vout.iter()
            .fold(0, |acc, vout| acc + vout.value)
    }

    pub fn get_rewards_value_and_address(&self) -> Vec<(u64, String)> {
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

    pub fn get_status(&self) -> &Status {
        &self.status
    }
}

impl VectorOutputs {
    pub fn get_scriptpubkey_address(&self) -> &Option<String> {
        &self.scriptpubkey_address
    }

    pub fn get_value(&self) -> u64 {
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
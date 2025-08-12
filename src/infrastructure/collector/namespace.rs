pub enum NameSpaceApi {
    Blocks(Option<u64>),
    _BlockByHash(String),
    _BlockByHashCoinbase(String),
    _BlockTxs(String),
    BlockTxids(String),
    TxById(String),
}

impl NameSpaceApi {
    pub fn get_uri_by_ns(&self) -> String {
        match self {
            NameSpaceApi::Blocks(from) => {
                from.map_or("blocks/".to_string(), |height| format!("blocks/{height}"))
            }
            NameSpaceApi::_BlockByHash(block_hash) => {
                format!("block/{block_hash}")
            }
            NameSpaceApi::_BlockByHashCoinbase(block_hash) => {
                format!("block/{block_hash}/txs/0")
            }
            NameSpaceApi::_BlockTxs(block_hash) => {
                format!("block/{block_hash}/txs")
            }
            NameSpaceApi::BlockTxids(block_hash) => {
                format!("block/{block_hash}/txids")
            }
            NameSpaceApi::TxById(tx_id) => {
                format!("tx/{tx_id}")
            }
        }
    }
}
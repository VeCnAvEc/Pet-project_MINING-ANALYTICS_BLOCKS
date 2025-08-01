pub enum NameSpaceApi {
    Blocks(u64),
    BlockByHash(String),
    BlockByHashCoinbase(String),
    BlockTxs(String),
    BlockTxids(String),
    TxById(String),
}

impl NameSpaceApi {
    pub fn get_uri_by_ns(&self) -> String {
        match self {
            NameSpaceApi::Blocks(height) => {
                format!("blocks/{height}")
            }
            NameSpaceApi::BlockByHash(block_hash) => {
                format!("block/{block_hash}")
            }
            NameSpaceApi::BlockByHashCoinbase(block_hash) => {
                format!("block/{block_hash}/txs/0")
            }
            NameSpaceApi::BlockTxs(block_hash) => {
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

pub fn build_url(basic_url: &str, ns: NameSpaceApi) -> String {
    let uri = ns.get_uri_by_ns();

    return format!("{basic_url}{uri}");
}
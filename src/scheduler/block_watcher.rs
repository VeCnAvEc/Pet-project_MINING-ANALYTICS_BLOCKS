use std::sync::Arc;
use reqwest::Client;
use crate::config::config::Config;

struct BlockWatcher {
    reqwest: Arc<Client>,
    config: Arc<Config>
}

impl BlockWatcher {
    pub fn new(client: Arc<Client>, config: Arc<Config>) -> Self {
        Self {
            reqwest: client,
            config
        }
    }

    pub async fn start_monitoring_new_blocks(&mut self) {

    }

    pub fn process_blocks() {

    }

    pub fn process_block() {

    }
}
use std::fs::File;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    api_url: String,
    interval_analytic_blocks: u64,
    rabbitmq_config: RabbitMqConfig
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabbitMqConfig {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    stream_name: String
}

impl RabbitMqConfig {
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    pub fn get_stream_name(&self) -> &str {
        &self.stream_name
    }
}

impl Config {
    pub fn new() -> Config {
        let default_path = "./config/config.json";
        let args: Vec<String> = env::args().collect();

        let config_path = if args.len() < 2 {
            default_path
        } else {
            args[1].as_str()
        };

        let file = File::open(config_path).expect("The file could not be opened");

        let reader = std::io::BufReader::new(file);
        let config: Config = from_reader(reader).expect("Couldn't read JSON");

        config
    }

    pub fn get_api_url(&self) -> &str {
        &self.api_url
    }

    pub fn get_interval_analytic_blocks(&self) -> u64 {
        self.interval_analytic_blocks
    }

    pub fn get_rabbitmq_config(&self) -> &RabbitMqConfig {
        &self.rabbitmq_config
    }
}
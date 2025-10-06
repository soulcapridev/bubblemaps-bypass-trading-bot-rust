use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub sol_wallet: String,
    pub private_key: String,
    pub target_wallets: Vec<String>,
    pub bsc_rpc_url: String,
    pub solana_rpc_url: String,
}

impl Config {
    pub fn load() -> Self {
        let config_str = fs::read_to_string("config.json").expect("Failed to read config");
        serde_json::from_str(&config_str).expect("Invalid config format")
    }
}

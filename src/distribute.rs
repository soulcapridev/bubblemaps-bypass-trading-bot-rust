use crate::config::Config;

pub async fn distribute_tokens(cfg: &Config) {
    println!("Distributing tokens to multiple wallets...");

    for wallet in &cfg.target_wallets {
        // Placeholder for transfer logic
        println!("Sending tokens to wallet: {}", wallet);
        // Implement transfer via SDK/API
    }
}

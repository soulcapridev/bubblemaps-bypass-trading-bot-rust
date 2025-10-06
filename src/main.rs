mod swap;
mod distribute;
mod utils;
mod config;

use tokio;

#[tokio::main]
async fn main() {
    println!("Starting Bubblemaps-Bypass Trading Bot");

    // Load configuration
    let cfg = config::Config::load();

    // Step 1: Buy SOL token at launch
    utils::buy_token(&cfg).await;

    // Step 2: Cross-chain swap (SOL -> BSC -> SOL with fresh wallet)
    swap::cross_chain_swap(&cfg).await;

    // Step 3: Distribute tokens to multiple wallets
    distribute::distribute_tokens(&cfg).await;

    println!("Operations completed");
}

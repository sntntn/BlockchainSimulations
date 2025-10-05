mod config;
mod models;
mod rpc;

use config::{get_rpc_url, load_env};
use models::SimpleBlock;
use rpc::fetch::fetch_latest_block;
use tokio;

fn print_block_info(block: &SimpleBlock) {
    println!("Blok broj: {}", block.number);
    println!("Hash bloka: {}", block.hash);
    println!("Timestamp: {}", block.timestamp);
    println!("Gas used: {}", block.gas_used);
    println!("Broj transakcija: {}", block.transactions.len());

    for tx in &block.transactions {
        println!("---");
        println!("Tx hash: {}", tx.hash);
        println!("From: {}", tx.from);
        println!("To: {}", tx.to.clone().unwrap_or_else(|| "N/A".into()));
        println!("Value: {}", tx.value);
        println!("Gas: {}", tx.gas);
        println!("Gas price: {}", tx.gas_price);
    }
}

#[tokio::main]
async fn main() {
    load_env();

    let rpc_url = get_rpc_url();

    let block = fetch_latest_block(&rpc_url).await;
    print_block_info(&block);
}

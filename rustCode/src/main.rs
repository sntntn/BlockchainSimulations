mod config;
mod models;
mod rpc;

use config::{get_testnet_rpc_url, get_mainnet_rpc_url, load_testnet_env};
use models::SimpleBlock;
use rpc::fetch::{fetch_latest_block, fetch_block_by_number};
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_testnet_env();

    let testnet_rpc_url = get_testnet_rpc_url();

    let block = fetch_latest_block(&testnet_rpc_url, true).await;
    print_block_info(&block);

    println!("----------------------");
    let mainnet_rpc_url = get_mainnet_rpc_url();
    println!("{}",mainnet_rpc_url);

    let block_response = fetch_block_by_number(&mainnet_rpc_url,"latest" , false).await?;
    println!("{}",block_response);

    Ok(())
}

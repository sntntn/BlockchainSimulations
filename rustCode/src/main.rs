mod config;
mod models;
mod rpc;
mod utils;

use config::{get_testnet_rpc_url, get_mainnet_rpc_url, load_testnet_env};
use rpc::fetch::{fetch_latest_block, fetch_block_by_number, fetch_last_5_blocks_and_receipts};
use utils::{print_block_info, hex_to_u64, analyze_max_gas_transaction};
use tokio;

//use crate::utils::print_transactions;

#[tokio::main]
async fn main() {
    load_testnet_env();

    let testnet_rpc_url = get_testnet_rpc_url();
    let mainnet_rpc_url = get_mainnet_rpc_url();


    let (block_testnet, block_mainnet) = tokio::join!(
        fetch_latest_block(&testnet_rpc_url, true),
        fetch_block_by_number(&mainnet_rpc_url, "latest", true)
    );

    println!("Testnet blok:");
    print_block_info(&block_testnet);
    //print_transactions(&block_testnet.transactions);
    println!("----------------------");
    println!("Mainnet blok:");
    print_block_info(&block_mainnet);

    let transactions = block_mainnet.transactions;
    //print_transactions(&transactions);

    analyze_max_gas_transaction(&mainnet_rpc_url, &transactions, &block_mainnet.gas_used).await;
    
    println!();
    println!("=======================");
    println!("Fetchujem 5 poslednjih blokova i njihove max gas transakcije...");
    println!("=======================\n");

    let latest_block_number = hex_to_u64(&block_mainnet.number);
    let summaries = fetch_last_5_blocks_and_receipts(mainnet_rpc_url, latest_block_number).await;

    println!("\n===== REZIME 5 BLOKOVA =====");
    for s in summaries {
        println!(
            "Blok {} | TX {} | Gas {} | {:.3}%",
            s.block_number, s.tx_hash, s.gas_used, s.percent_in_block
        );
    }

}
mod config;
mod models;
mod rpc;
mod utils;

use config::{get_testnet_rpc_url, get_mainnet_rpc_url, load_testnet_env};
use rpc::fetch::{fetch_latest_block, fetch_block_by_number, fetch_transaction_receipt};
use utils::{print_block_info, print_transactions, find_max_gas_transaction};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_testnet_env();

    let testnet_rpc_url = get_testnet_rpc_url();

    let block = fetch_latest_block(&testnet_rpc_url, true).await;
    print_block_info(&block);

    println!("----------------------");
    let mainnet_rpc_url = get_mainnet_rpc_url();
    println!("{}",mainnet_rpc_url);

    let block_response = fetch_block_by_number(&mainnet_rpc_url,"latest" , true).await;
    print_block_info(&block_response);
    let transactions = block_response.transactions;
    print_transactions(&transactions);

    println!("----------------------");
    
    if let Some(max_tx) = find_max_gas_transaction(&transactions){
        println!("Transakcija sa najvecim gas limitom: ");
        println!("Hash {}", max_tx.hash);
        println!("Gas limit: (hex): {}", max_tx.gas);

        let receipt = fetch_transaction_receipt(&mainnet_rpc_url, &max_tx.hash).await;
        //println!("{:?}", receipt);

        let tx_gas_used = 
            u64::from_str_radix(receipt.gas_used.trim_start_matches("0x"), 16).unwrap_or(0);
        let block_gas_used =
            u64::from_str_radix(block_response.gas_used.trim_start_matches("0x"), 16).unwrap_or(1);

        let percent_of_block = (tx_gas_used as f64 / block_gas_used as f64) * 100.0;

        println!("Gas potrosen od ove transakcije: {}", tx_gas_used);
        println!("Gas potrosen u bloku: {}", block_gas_used);
        println!("Procenat potrosnje u bloku: {:.6}%", percent_of_block);
    } else {
            println!("Nema transakcija u ovom bloku");
    }

    Ok(())
}

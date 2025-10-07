mod config;
mod models;
mod rpc;

use config::{get_testnet_rpc_url, get_mainnet_rpc_url, load_testnet_env};
use models::SimpleBlock;
use rpc::fetch::{fetch_latest_block, fetch_block_by_number};
use tokio;

use crate::{models::SimpleTransaction, rpc::fetch::fetch_transaction_receipt};

fn print_block_info(block: &SimpleBlock) {
    println!("Blok broj: {}", block.number);
    println!("Hash bloka: {}", block.hash);
    println!("Timestamp: {}", block.timestamp);
    println!("Gas used: {}", block.gas_used);
    println!("Broj transakcija: {}", block.transactions.len());
    println!("Transakcije: ");
    //print_transactions(&block.transactions);
}

fn print_transactions(transactions: &[SimpleTransaction]) {
    for tx in transactions {
        println!("---");
        println!("Tx hash: {}", tx.hash);
        println!("From: {}", tx.from);
        println!("To: {}", tx.to.clone().unwrap_or_else(|| "N/A".into()));
        println!("Value: {}", tx.value);
        println!("Gas: {}", tx.gas);
        println!("Gas price: {}", tx.gas_price);
    }

    println!("----------------------");
    println!("ispisano je {} transakcija", transactions.len());
}

fn find_max_gas_transaction(transactions: &[SimpleTransaction]) -> Option<&SimpleTransaction> {
    transactions
        .iter()
        .max_by_key(|tx| u64::from_str_radix(tx.gas.trim_start_matches("0x"), 16).unwrap_or(0))
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

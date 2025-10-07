mod config;
mod models;
mod rpc;
mod utils;

use config::{get_testnet_rpc_url, get_mainnet_rpc_url, load_testnet_env};
use rpc::fetch::{fetch_latest_block, fetch_block_by_number, fetch_transaction_receipt};
use utils::{print_block_info, print_transactions, find_max_gas_transaction, hex_to_u64, calculate_gas_percentage, consume_and_calculate_gas};
use tokio;

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
    println!("----------------------");
    println!("Mainnet blok:");
    print_block_info(&block_mainnet);

    let transactions = block_mainnet.transactions;
    print_transactions(&transactions);

    println!("----------------------");
    
    if let Some(max_tx) = find_max_gas_transaction(&transactions){
        println!("Transakcija sa najvecim gas limitom: ");
        println!("Hash {}", max_tx.hash);
        println!("Gas limit: (hex): {}", max_tx.gas);
        println!();

        let receipt = fetch_transaction_receipt(&mainnet_rpc_url, &max_tx.hash).await;
        //println!("{:?}", receipt);    // OVO MOZE
        let tx_gas_used = hex_to_u64(&receipt.gas_used);
        let block_gas_used = hex_to_u64(&block_mainnet.gas_used);
        let percent_of_block = calculate_gas_percentage(tx_gas_used, block_gas_used);
    
        println!("Gas potrosen od ove transakcije: {}", tx_gas_used);
        println!("Gas potrosen u bloku: {}", block_gas_used);
        println!("Procenat potrosnje u bloku: {:.6}%", percent_of_block);
        println!();
        
        println!("Funkcija uzima ownership");
        let percent_of_block1 = consume_and_calculate_gas(receipt, block_mainnet.gas_used);
        println!("Procenat potrosnje u bloku: {:.6}%", percent_of_block1);
        //println!("{:?}", receipt);                                         //OVO NE MOZE jer je promenjen owner
        //println!("{}",block_response.gas_used);
        //let tx_gas_used1 = hex_to_u64(&receipt.gas_used);
        //let block_gas_used1 = hex_to_u64(&block_response.gas_used);
    } else {
            println!("Nema transakcija u ovom bloku");
    }
}

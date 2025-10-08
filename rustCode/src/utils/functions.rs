use crate::models::{SimpleTransaction, TransactionReceipt};
use crate::rpc::fetch::fetch_transaction_receipt;

pub fn find_max_gas_transaction(transactions: &[SimpleTransaction]) -> Option<&SimpleTransaction> {
    transactions
        .iter()
        .max_by_key(|tx| u64::from_str_radix(tx.gas.trim_start_matches("0x"), 16).unwrap_or(0))
}

pub fn hex_to_u64(hex: &str) -> u64 {
    u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0)
}

pub fn calculate_gas_percentage(tx_gas_used: u64, block_gas_used: u64) -> f64 {
    if block_gas_used == 0 {
        return 0.0;
    }
    (tx_gas_used as f64 / block_gas_used as f64) * 100.0
}

pub fn consume_and_calculate_gas(receipt: TransactionReceipt, block_gas_used_hex: String) -> f64 {
    let tx_gas_used = hex_to_u64(&receipt.gas_used);
    let block_gas_used = hex_to_u64(&block_gas_used_hex);
    calculate_gas_percentage(tx_gas_used, block_gas_used)
}

pub async fn analyze_max_gas_transaction(
    mainnet_rpc_url: &str,
    transactions: &[SimpleTransaction],
    block_gas_used_hex: &str,
) {
    println!("----------------------");

    if let Some(max_tx) = find_max_gas_transaction(transactions) {
        println!("Transakcija sa najvecim gas limitom: ");
        println!("Hash {}", max_tx.hash);
        println!("Gas limit: {}", hex_to_u64(&max_tx.gas));
        println!();

        let receipt = fetch_transaction_receipt(mainnet_rpc_url, &max_tx.hash).await;
        //println!("{:?}", receipt);                                         // OVO MOZE
        let tx_gas_used = hex_to_u64(&receipt.gas_used);
        let block_gas_used = hex_to_u64(block_gas_used_hex);
        let percent_of_block = calculate_gas_percentage(tx_gas_used, block_gas_used);

        println!("Gas potrosen od ove transakcije: {}", tx_gas_used);
        println!("Gas potrosen u bloku: {}", block_gas_used);
        println!("Procenat potrosnje u bloku: {:.6}%", percent_of_block);
        println!();

        // Ownership primer
        println!("Ponovljeno izracunavanje preko dodele ownership-a");
        let percent_of_block1 = consume_and_calculate_gas(receipt, block_gas_used_hex.to_string());
        println!("Procenat potrosnje u bloku: {:.6}%", percent_of_block1);
        //println!("{:?}", receipt);                                         //OVO NE MOZE jer je promenjen owner
        //println!("{}",block_mainnet.gas_used);
        //let tx_gas_used1 = hex_to_u64(&receipt.gas_used);
        //let block_gas_used1 = hex_to_u64(&block_mainnet.gas_used);

    } else {
        println!("Nema transakcija u ovom bloku");
    }
}
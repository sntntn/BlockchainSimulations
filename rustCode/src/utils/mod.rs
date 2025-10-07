use crate::models::{SimpleBlock, SimpleTransaction};

pub fn print_block_info(block: &SimpleBlock) {
    println!("Blok broj: {}", block.number);
    println!("Hash bloka: {}", block.hash);
    println!("Timestamp: {}", block.timestamp);
    println!("Gas used: {}", block.gas_used);
    println!("Broj transakcija: {}", block.transactions.len());
}

pub fn print_transactions(transactions: &[SimpleTransaction]) {
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
    println!("Ispisano je {} transakcija", transactions.len());
}

pub fn find_max_gas_transaction(transactions: &[SimpleTransaction]) -> Option<&SimpleTransaction> {
    transactions
        .iter()
        .max_by_key(|tx| u64::from_str_radix(tx.gas.trim_start_matches("0x"), 16).unwrap_or(0))
}

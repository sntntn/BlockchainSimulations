use crate::models::{SimpleTransaction, TransactionReceipt};

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
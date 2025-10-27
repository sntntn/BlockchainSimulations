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

pub fn borrow_and_calculate_gas(receipt: &TransactionReceipt, block_gas_used_hex: &str) -> f64 {
    let tx_gas_used = hex_to_u64(&receipt.gas_used);
    let block_gas_used = hex_to_u64(block_gas_used_hex);
    calculate_gas_percentage(tx_gas_used, block_gas_used)
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
        
        let tx_gas_used = hex_to_u64(&receipt.gas_used);
        let block_gas_used = hex_to_u64(block_gas_used_hex);
        println!("Gas potrosen od ove transakcije: {}", tx_gas_used);
        println!("Gas potrosen u bloku: {}", block_gas_used);

        let percent_of_block = calculate_gas_percentage(tx_gas_used, block_gas_used);
        println!("Procenat potrosnje u bloku: {:.6}%", percent_of_block);
        //println!("{:?}", receipt);                                         // DOZVOLJENO

        println!();
        println!("Ponovljeno izracunavanje preko borrow reference");
        let percent_of_block1 = borrow_and_calculate_gas(&receipt, block_gas_used_hex);
        println!("Procenat potrosnje u bloku: {:.6}%", percent_of_block1);
        //println!("{:?}", receipt);                                         // DOZVOLJENO
        println!();

        // Ownership primer
        println!("Ponovljeno izracunavanje preko dodele ownership-a");
        let percent_of_block1 = consume_and_calculate_gas(receipt, block_gas_used_hex.to_string());
        println!("Procenat potrosnje u bloku: {:.6}%", percent_of_block1);
        //println!("{:?}", receipt);                                         //NIJE DOZVOLJENO jer je promenjen owner
        //println!("{:?}", block_gas_used_hex);                              //DOZVOLJENO jer se napravila kopija na hip-u

    } else {
        println!("Nema transakcija u ovom bloku");
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_hex_to_u_64(){
        let result = hex_to_u64("0x1a");
        assert_eq!(result,26);
    }
    #[test]
    fn test_hex_to_u64_invalid_hex() {
        let result = hex_to_u64("not_a_hex");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calculate_gas_percentage(){
        let result = calculate_gas_percentage(50, 200);
        assert!(
            (result-25.0).abs() < 1e-6,
            "25% test failed, result is {result}"
        );

        let zero_edge_case = calculate_gas_percentage(1000, 0);
        assert_eq!(
            zero_edge_case, 0.0,
            "zero edge case test failed"
        );
    }

    #[test]
    fn test_find_max_gas_transaction() {
        let tx1 = SimpleTransaction {
            hash: "0x1".to_string(),
            from: "0xabc".to_string(),
            to: Some("0xdef".to_string()),
            value: "0x10".to_string(),
            gas: "0x10".to_string(),       // 16
            gas_price: "0x1".to_string(),
        };
        let tx2 = SimpleTransaction {
            hash: "0x2".to_string(),
            from: "0xabc".to_string(),
            to: Some("0xdef".to_string()),
            value: "0x20".to_string(),
            gas: "0x20".to_string(),       // 32
            gas_price: "0x1".to_string(),
        };
        let tx3 = SimpleTransaction {
            hash: "0x3".to_string(),
            from: "0xabc".to_string(),
            to: Some("0xdef".to_string()),
            value: "0x30".to_string(),
            gas: "0x15".to_string(),       // 21
            gas_price: "0x1".to_string(),
        };

        let transactions = vec![tx1, tx2, tx3];
        let result = find_max_gas_transaction(&transactions).unwrap();

        assert_eq!(result.hash, "0x2");
    }

    #[test]
    fn test_realistic_transaction_data() {
        let tx = SimpleTransaction {
            hash: "0x8e2e8a4f08807b2e94fd87eb1a88fd445b25f2b2ae04a604015ab1f47a60a700".to_string(),
            from: "0xfbe5e8d44e14cdafb4932090806d27c80c5ffbaa".to_string(),
            to: Some("0xfbe5e8d44e14cdafb4932090806d27c80c5ffbaa".to_string()),
            value: "0xde0b6b3a7640000".to_string(), // 1 ether u wei
            gas: "0x5208".to_string(),              // 21000
            gas_price: "0x1".to_string(),
        };

        let gas_as_u64 = hex_to_u64(&tx.gas);
        assert_eq!(gas_as_u64, 21000);
    }

}
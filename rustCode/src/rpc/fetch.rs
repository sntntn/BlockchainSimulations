use crate::models::{RPCResponseBlock, RPCResponseReceipt, SimpleBlock, TransactionReceipt, TxSummary};
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use crate::utils::{find_max_gas_transaction, hex_to_u64, calculate_gas_percentage};

pub async fn fetch_latest_block(rpc_url: &str, transaction_bool: bool) -> SimpleBlock {
    let req_body = json!( {
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": ["latest", transaction_bool],
        "id": 1
    });

    let client = Client::new();
    let resp = client
        .post(rpc_url)
        .json(&req_body)
        .send()
        .await
        .expect("Greska pri slanju zahteva");

    let resp_json: RPCResponseBlock = resp.json().await.expect("Greska pri parsiranju odgovora");

    resp_json.result
}

pub async fn fetch_block_by_number(rpc_url: &str, block_number: &str, transaction_bool: bool
) -> SimpleBlock {
    let req_body = json!( {
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": [block_number, transaction_bool],
        "id": 1
    });

    let client = Client::new();
    let resp = client
        .post(rpc_url)
        .json(&req_body)
        .send()
        .await
        .expect("Greska pri slanju zahteva");
    
    let resp_json: RPCResponseBlock = resp.json().await.expect("Greska pri parsiranju odgovora");

    resp_json.result
}

pub async fn fetch_transaction_receipt(rpc_url: &str, tx_hash: &str) -> TransactionReceipt {
    let req_body = json!({
        "jsonrpc": "2.0",
        "method": "eth_getTransactionReceipt",
        "params": [tx_hash],
        "id": 1
    });

    let client = Client::new();

    let resp = client
        .post(rpc_url)
        .json(&req_body)
        .send()
        .await
        .expect("Greska pri slanju zahteva");

    let receipt_resp: RPCResponseReceipt = resp.json().await.expect("Greska pri parsiranju odgovora");

    receipt_resp.result
}

pub async fn fetch_last_5_blocks_and_receipts(rpc_url: String, latest_block_number: u64) 
-> Vec<TxSummary> {
    let rpc_arc = Arc::new(rpc_url);
    
    let mut tasks = Vec::new();

    for i in 0..5 {
        let rpc = Arc::clone(&rpc_arc);
        let block_number = latest_block_number.saturating_sub(i); // da ne padne ispod 0

        let task = tokio::spawn(async move {
            let block_number_hex = format!("0x{:x}", block_number);
            let block = fetch_block_by_number(&rpc, &block_number_hex, true).await;

            if let Some(max_tx) = find_max_gas_transaction(&block.transactions) {
                let receipt = fetch_transaction_receipt(&rpc, &max_tx.hash).await;
                let tx_gas_used = hex_to_u64(&receipt.gas_used);
                let block_gas_used = hex_to_u64(&block.gas_used);
                let percent = calculate_gas_percentage(tx_gas_used, block_gas_used);

                println!(
                    "Blok {} | Max TX: {} | Gas: {} | %. u bloku: {:.3}%",
                    block.number, max_tx.hash, tx_gas_used, percent
                );

                Some(TxSummary {
                    block_number: block.number.clone(),
                    tx_hash: max_tx.hash.clone(),
                    gas_used: tx_gas_used,
                    percent_in_block: percent,
                })            
            } else {
                println!("Blok {} nema transakcije.", block.number);
                None
            }
        });

        tasks.push(task);
    }

    let results = futures::future::join_all(tasks).await;

    // filtriram samo uspesne
    results
        .into_iter()
        .filter_map(|res| res.ok().flatten())
        .collect()
}
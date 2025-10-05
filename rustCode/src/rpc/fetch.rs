use crate::models::{RPCResponse, SimpleBlock};
use reqwest::Client;
use serde_json::{json, Value};

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

    let resp_json: RPCResponse = resp.json().await.expect("Greska pri parsiranju odgovora");

    resp_json.result
}

pub async fn fetch_block_by_number(rpc_url: &str, block_number: &str, transaction_bool: bool
) -> Result<String, Box<dyn std::error::Error>> {
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
        .await?;
    
    let tekst=resp.text().await?;

    Ok(tekst)
}
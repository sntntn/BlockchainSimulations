use std::{env, path::Path};
use dotenvy::from_path;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio;


#[derive(Debug, Deserialize)]
struct RPCResponse {
    jsonrpc: String,
    id: u64,
    result: SimpleBlock,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SimpleBlock {
    number: String,
    hash: String,
    timestamp: String,
    gas_used: String,
    transactions: Vec<SimpleTransaction>,
}

#[derive(Debug, Deserialize)]
struct SimpleTransaction {
    hash: String,
    from: String,
    to: Option<String>,
    value: String,
    gas: String,
    
    #[serde(rename = "gasPrice")]
    gas_price: String,
}


#[tokio::main]
async fn main() {

    let env_path = Path::new("../code/.env");
    from_path(env_path).expect("Nije moguce ucitati .env fajl sa date putanje");

    let rpc_url =  env::var("RPC_URL").expect("RPC_URL nije definisan");

    let req_body = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": ["latest", true],
        "id": 1
    });

    let client = Client::new();
    let resp = client
        .post(rpc_url)
        .json(&req_body)
        .send()
        .await
        .expect("Greska pri slanju zahteva");

    // let resp_text = resp.text().await.expect("Greska pri citanju odgovora");
    // println!("Odgovor sa servera:\n{}", resp_text);

    let resp_json: RPCResponse = resp
        .json()
        .await
        .expect("Greska pri parsiranju odgovora");


    let block = resp_json.result;


    println!("Blok broj: {}", block.number);
    println!("Hash bloka: {}", block.hash);
    println!("Timestamp: {}", block.timestamp);
    println!("Gas used: {}", block.gas_used);
    println!("Broj transakcija: {}", block.transactions.len());

    for tx in block.transactions {
        println!("---");
        println!("Tx hash: {}", tx.hash);
        println!("From: {}", tx.from);
        println!("To: {}", tx.to.unwrap_or_else(|| "N/A".into()));
        println!("Value: {}", tx.value);
        println!("Gas: {}", tx.gas);
        println!("Gas price: {}", tx.gas_price);
    }

}

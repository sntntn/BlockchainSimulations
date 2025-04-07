use reqwest::Client;
use serde_json::json;
use crate::models::{RPCResponse, SimpleBlock};

pub async fn fetch_latest_block(rpc_url: &str) -> SimpleBlock {
    let req_body = json!( {
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

    let resp_json: RPCResponse = resp
        .json()
        .await
        .expect("Greska pri parsiranju odgovora");

    resp_json.result
}

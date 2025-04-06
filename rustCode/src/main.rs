use std::{env, path::Path};
use dotenvy::from_path;
use reqwest::Client;
use serde_json::json;
use tokio;


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

    let resp_text = resp.text().await.expect("Greska pri citanju odgovora");
    println!("Odgovor sa servera:\n{}", resp_text);
}

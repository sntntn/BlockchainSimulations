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

// #[no_mangle]
// pub extern "C" fn fetch_transactions(rpc_url: *const c_char) -> *mut c_char {
//     // Pretvaranje C stringa u Rust string
//     let c_str = unsafe { CString::from_raw(rpc_url as *mut c_char) };
//     let rpc_url_str = c_str.to_str().unwrap();

//     // Pozivanje fetch_latest_block funkcije za dobijanje najnovijeg bloka
//     let future = tokio::runtime::Runtime::new().unwrap().block_on(fetch_latest_block(rpc_url_str));

//     // Prikupljanje transakcija iz rezultata
//     let mut transaction_data = String::new();
//     for tx in &future.transactions {
//         transaction_data.push_str(&format!("Tx hash: {}\nFrom: {}\nTo: {}\nValue: {}\nGas: {}\n\n",
//                                           tx.hash, tx.from, tx.to.clone().unwrap_or_else(|| "N/A".into()),
//                                           tx.value, tx.gas));
//     }

//     let c_str_result = CString::new(transaction_data).unwrap();

//     // Vraćanje C stringa koji Go može koristiti
//     c_str_result.into_raw()
// }
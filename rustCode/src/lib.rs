pub mod models;
pub mod rpc;
pub mod utils;

use crate::rpc::fetch::{fetch_latest_block, fetch_last_5_blocks_and_receipts};
use std::ffi::CString;
use std::os::raw::c_char;
use crate::utils::hex_to_u64;

#[no_mangle]
pub extern "C" fn fetch_transactions(rpc_url: *const c_char) -> *mut c_char {
    // Pretvaranje C stringa u Rust string
    let c_str = unsafe { std::ffi::CStr::from_ptr(rpc_url) };
    let rpc_url_str = c_str.to_str().unwrap();
    // Pozivanje fetch_latest_block funkcije za dobijanje najnovijeg bloka
    let future = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(fetch_latest_block(rpc_url_str, true));

    // Prikupljanje transakcija iz rezultata
    let mut transaction_data = String::new();
    for tx in &future.transactions {
        transaction_data.push_str(&format!(
            "Tx hash: {}\nFrom: {}\nTo: {}\nValue: {}\nGas: {}\n\n",
            tx.hash,
            tx.from,
            tx.to.clone().unwrap_or_else(|| "N/A".into()),
            tx.value,
            tx.gas
        ));
    }

    let c_str_result = CString::new(transaction_data).unwrap();

    // Vracanje C stringa koji Go moze koristiti
    c_str_result.into_raw()
}

#[no_mangle]
pub extern "C" fn fetch_last_5_blocks(rpc_url: *const c_char) -> *mut c_char {
    let c_rpc = unsafe { std::ffi::CStr::from_ptr(rpc_url) };
    let rpc_url_str = c_rpc.to_str().unwrap();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let latest_block = runtime.block_on(fetch_latest_block(rpc_url_str, true));
    let latest_block_number = hex_to_u64(&latest_block.number);

    let summaries = runtime.block_on(fetch_last_5_blocks_and_receipts(rpc_url_str.to_string(), latest_block_number));

    let json_str = serde_json::to_string(&summaries).unwrap();
    let c_str_result = CString::new(json_str).unwrap();
    c_str_result.into_raw()
}
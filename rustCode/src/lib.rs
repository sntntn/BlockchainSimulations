pub mod models;
pub mod rpc;

use crate::rpc::fetch::fetch_latest_block;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn fetch_transactions(rpc_url: *const c_char) -> *mut c_char {
    // Pretvaranje C stringa u Rust string
    let c_str = unsafe { std::ffi::CStr::from_ptr(rpc_url) };
    let rpc_url_str = c_str.to_str().unwrap();
    // Pozivanje fetch_latest_block funkcije za dobijanje najnovijeg bloka
    let future = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(fetch_latest_block(rpc_url_str));

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

    // Vracanje C stringa koji Go moye koristiti
    c_str_result.into_raw()
}

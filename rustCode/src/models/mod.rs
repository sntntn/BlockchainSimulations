use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RPCResponseBlock {
    pub result: SimpleBlock,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleBlock {
    pub number: String,
    pub hash: String,
    pub timestamp: String,
    pub gas_used: String,
    pub transactions: Vec<SimpleTransaction>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SimpleTransaction {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub gas: String,

    #[serde(rename = "gasPrice")]
    pub gas_price: String,
}

#[derive(Debug, Deserialize)]
pub struct RPCResponseReceipt {
    pub result: TransactionReceipt,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub block_hash: String,
    pub block_number: String,
    pub transaction_hash: String,
    pub gas_used: String,
    pub cumulative_gas_used: String,
}

pub struct TxSummary {
    pub block_number: String,
    pub tx_hash: String,
    pub gas_used: u64,
    pub percent_in_block: f64,
}
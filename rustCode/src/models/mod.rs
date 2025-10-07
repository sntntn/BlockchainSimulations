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
    pub transaction_hash: String,
    pub gas_used: String,
    pub cumulative_gas_used: String,
}
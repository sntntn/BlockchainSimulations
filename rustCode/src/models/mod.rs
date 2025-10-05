use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RPCResponse {
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

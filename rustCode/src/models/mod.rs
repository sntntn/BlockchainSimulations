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
pub struct RPCResponseBlockTransactions {
    pub result: BlockTransactions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransactions {
    pub block_hash: String,
    pub transactions: Vec<SimpleTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct RPCResponseReceipt {
    pub result: TransactionReceipt,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    transaction_hash: String,
    gas_used: String,
    cumulative_gas_used: String,
}
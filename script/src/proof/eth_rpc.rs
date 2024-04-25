use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EthGetProofResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: ProofResult,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProofResult {
    pub address: String,
    pub account_proof: Vec<String>,
    pub balance: String,
    pub code_hash: String,
    pub nonce: String,
    pub storage_hash: String,
    pub storage_proof: Vec<StorageProof>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageProof {
    pub key: String,
    pub value: String,
    pub proof: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EthGetBlockByNumberResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: BlockResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockResult {
    pub base_fee_per_gas: String,
    pub blob_gas_used: String,
    pub difficulty: String,
    pub excess_blob_gas: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub hash: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: String,
    pub parent_beacon_block_root: String,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: String,
    pub state_root: String,
    pub timestamp: String,
    pub total_difficulty: String,
    pub transactions: Vec<String>,
    pub transactions_root: String,
    pub uncles: Vec<String>,
    pub withdrawals: Vec<Withdrawal>,
    pub withdrawals_root: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    pub index: String,
    pub validator_index: String,
    pub address: String,
    pub amount: String,
}

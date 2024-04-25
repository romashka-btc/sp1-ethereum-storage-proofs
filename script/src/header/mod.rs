use crate::proof::eth_rpc::BlockResult;
use crate::utils::parse_hex_to_u64;
use anyhow::{Error, Result};
use rlp::{Encodable, RlpStream};

#[derive(Debug, Clone)]
pub struct EvmBlockHeader {
    pub parent_hash: String,
    pub uncle_hash: String,
    pub coinbase: String,
    pub state_root: String,
    pub transactions_root: String,
    pub receipts_root: String,
    pub logs_bloom: String,
    pub difficulty: u64,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: String,
    pub mix_hash: String,
    pub nonce: String,
    pub base_fee_per_gas: Option<u64>,
    pub withdrawals_root: Option<String>,
    pub blob_gas_used: Option<u64>,
    pub excess_blob_gas: Option<u64>,
    pub parent_beacon_block_root: Option<String>,
}

impl Encodable for EvmBlockHeader {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(
            15 + self.base_fee_per_gas.is_some() as usize
                + self.withdrawals_root.is_some() as usize
                + self.blob_gas_used.is_some() as usize
                + self.excess_blob_gas.is_some() as usize
                + self.parent_beacon_block_root.is_some() as usize,
        );

        s.append(&safe_hex_decode(&self.parent_hash));
        s.append(&safe_hex_decode(&self.uncle_hash));
        s.append(&safe_hex_decode(&self.coinbase));
        s.append(&safe_hex_decode(&self.state_root));
        s.append(&safe_hex_decode(&self.transactions_root));
        s.append(&safe_hex_decode(&self.receipts_root));
        s.append(&safe_hex_decode(&self.logs_bloom));

        // Numeric fields can be appended directly if they are already u64
        s.append(&self.difficulty);
        s.append(&self.number);
        s.append(&self.gas_limit);
        s.append(&self.gas_used);
        s.append(&self.timestamp);

        s.append(&safe_hex_decode(&self.extra_data));
        s.append(&safe_hex_decode(&self.mix_hash));
        s.append(&safe_hex_decode(&self.nonce));

        if let Some(base_fee) = self.base_fee_per_gas {
            s.append(&base_fee);
        }

        if let Some(ref withdrawals_root) = self.withdrawals_root {
            s.append(&safe_hex_decode(withdrawals_root));
        }

        if let Some(blob_gas_used) = self.blob_gas_used {
            s.append(&blob_gas_used);
        }

        if let Some(excess_blob_gas) = self.excess_blob_gas {
            s.append(&excess_blob_gas);
        }

        if let Some(ref parent_beacon_block_root) = self.parent_beacon_block_root {
            s.append(&safe_hex_decode(parent_beacon_block_root));
        }
    }
}

impl EvmBlockHeader {
    pub fn from_block_result(result: BlockResult) -> Result<Self, Error> {
        Ok(EvmBlockHeader {
            parent_hash: result.parent_hash[2..].to_string(),
            uncle_hash: result.sha3_uncles[2..].to_string(),
            coinbase: result.miner[2..].to_string(),
            state_root: result.state_root[2..].to_string(),
            transactions_root: result.transactions_root[2..].to_string(),
            receipts_root: result.receipts_root[2..].to_string(),
            logs_bloom: result.logs_bloom[2..].to_string(),
            difficulty: parse_hex_to_u64(&result.difficulty)?,
            number: parse_hex_to_u64(&result.number)?,
            gas_limit: parse_hex_to_u64(&result.gas_limit)?,
            gas_used: parse_hex_to_u64(&result.gas_used)?,
            timestamp: parse_hex_to_u64(&result.timestamp)?,
            extra_data: result.extra_data[2..].to_string(),
            mix_hash: result.mix_hash[2..].to_string(),
            nonce: result.nonce[2..].to_string(),
            base_fee_per_gas: Some(parse_hex_to_u64(&result.base_fee_per_gas)?),
            withdrawals_root: Some(result.withdrawals_root[2..].to_string()),
            blob_gas_used: Some(parse_hex_to_u64(&result.blob_gas_used)?),
            excess_blob_gas: Some(parse_hex_to_u64(&result.blob_gas_used)?),
            parent_beacon_block_root: Some(result.parent_beacon_block_root[2..].to_string()),
        })
    }
}

pub fn safe_hex_decode(s: &str) -> Vec<u8> {
    // Ensure the string is without the '0x' prefix
    let s = if let Some(s) = s.strip_prefix("0x") {
        s
    } else {
        s
    };

    // Pad the string with a leading zero if it has an odd length
    let s = if s.len() % 2 != 0 {
        format!("0{}", s)
    } else {
        s.to_string()
    };

    hex::decode(s).unwrap()
}

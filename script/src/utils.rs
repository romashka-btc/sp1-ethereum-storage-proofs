use rlp::Rlp;
use serde_json::Result as SerdeResult;

#[allow(dead_code)]
pub enum Block {
    Latest,
    Number(u64),
}
pub fn odd_to_even_hex(hex: &str) -> String {
    if hex.len() % 2 == 0 {
        hex.to_string()
    } else {
        format!("0{}", hex)
    }
}

pub fn get_key_ptrs(proof: Vec<&str>) -> Vec<usize> {
    let mut result = Vec::<usize>::new();
    let mut key_index = 0;

    for (i, p) in proof.iter().enumerate() {
        let bytes = hex::decode(&p[2..]).expect("Decoding failed");
        let mut in_res: Vec<String> = Vec::new();
        let decoded_list = Rlp::new(&bytes);
        for value in decoded_list.iter() {
            let hex_representation = format!("0x{}", hex::encode(value.data().unwrap()));
            in_res.push(hex_representation);
        }

        if in_res.len() > 2 {
            //branch node
            result.push(key_index);
            key_index += 1;
        } else if i != proof.len() - 1 && in_res.len() == 2 {
            //extension node
            let extension = &in_res[0][2..];
            let bytes = hex::decode(extension).expect("Decoding failed");
            let decoded: String = rlp::decode(&bytes).expect("Decoding failed");
            result.push(key_index);
            key_index += decoded.len();
        } else if i == proof.len() - 1 && in_res.len() == 2 {
            //leaf node
            result.push(key_index);
        }
    }
    result
}

pub fn parse_json<T: serde::de::DeserializeOwned>(json_str: &str) -> SerdeResult<T> {
    serde_json::from_str::<T>(json_str)
}

pub fn parse_hex_to_u64(hex: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(&hex[2..], 16)
}

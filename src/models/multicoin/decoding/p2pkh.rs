use bs58::Alphabet;
use thiserror::Error;
use crate::utils;


#[derive(Error, Debug)]
pub enum P2PKHError {
    #[error("Invalid P2PKH structure")]
    InvalidStructure(String)
}

pub fn decode(bytes: &[u8], version: u8) -> Result<String, P2PKHError> {
    let bytes_len = bytes.len();
    if bytes_len < 3 {
        return Err(P2PKHError::InvalidStructure("len < 3".to_string()));
    }

    if bytes[..2] != [0x76, 0xa9] {
        return Err(P2PKHError::InvalidStructure("invalid header".to_string()));
    }

    let len = bytes[2] as usize;
    let expected_len = 3 + len + 2;

    if bytes_len != expected_len {
        return Err(P2PKHError::InvalidStructure(format!("invalid length ({bytes_len:?} != {expected_len:?})")));
    }

    if bytes[bytes_len - 2..bytes_len] != [0x88, 0xac] {
        return Err(P2PKHError::InvalidStructure("invalid end".to_string()));
    }

    let pub_key_hash = &bytes[3..3 + len];

    let mut full = pub_key_hash.to_vec();
    full.insert(0, version);

    let full_checksum = utils::sha256::hash(utils::sha256::hash(full.clone()));

    full.extend_from_slice(&full_checksum[..4]);

    let value = bs58::encode(full)
        .with_alphabet(Alphabet::BITCOIN)
        .into_string();

    Ok(value)
}

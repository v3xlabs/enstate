use bs58::Alphabet;
use thiserror::Error;
use crate::utils;


#[derive(Error, Debug)]
pub enum P2SHError {
    #[error("Invalid P2SH structure")]
    InvalidStructure(String)
}

pub fn decode(bytes: &[u8], version: u8) -> Result<String, P2SHError> {
    let bytes_len = bytes.len();
    if bytes_len < 2 {
        return Err(P2SHError::InvalidStructure("len < 2".to_string()));
    }

    if bytes[0] != 0xa9 {
        return Err(P2SHError::InvalidStructure("invalid header".to_string()));
    }

    let len = bytes[1] as usize;
    let expected_len = 2 + len + 1;

    if bytes_len != expected_len {
        return Err(P2SHError::InvalidStructure(format!("invalid length ({bytes_len:?} != {expected_len:?})")));
    }

    if bytes[bytes_len - 1] != 0x87 {
        return Err(P2SHError::InvalidStructure("invalid end".to_string()));
    }

    let script_hash = &bytes[2..2 + len];

    let mut full = script_hash.to_vec();
    full.insert(0, version);

    let full_checksum = utils::sha256::hash(utils::sha256::hash(full.clone()));

    full.extend_from_slice(&full_checksum[..4]);

    let value = bs58::encode(full)
        .with_alphabet(Alphabet::BITCOIN)
        .into_string();

    Ok(value)
}

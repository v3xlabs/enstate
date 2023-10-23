use bs58::Alphabet;

use crate::utils;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct P2SHDecoder {
    pub version: u8,
}

impl MulticoinDecoder for P2SHDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let bytes_len = data.len();
        if bytes_len < 2 {
            return Err(MulticoinDecoderError::InvalidStructure("len < 2".to_string()));
        }
    
        if data[0] != 0xa9 {
            return Err(MulticoinDecoderError::InvalidStructure("invalid header".to_string()));
        }
    
        let len = data[1] as usize;
        let expected_len = 2 + len + 1;
    
        if bytes_len != expected_len {
            return Err(MulticoinDecoderError::InvalidStructure(format!("invalid length ({bytes_len:?} != {expected_len:?})")));
        }
    
        if data[bytes_len - 1] != 0x87 {
            return Err(MulticoinDecoderError::InvalidStructure("invalid end".to_string()));
        }
    
        let script_hash = &data[2..2 + len];
    
        let mut full = script_hash.to_vec();
        full.insert(0, self.version);
    
        let full_checksum = utils::sha256::hash(utils::sha256::hash(full.clone()));
    
        full.extend_from_slice(&full_checksum[..4]);
    
        let value = bs58::encode(full)
            .with_alphabet(Alphabet::BITCOIN)
            .into_string();
    
        Ok(value)
    }
}

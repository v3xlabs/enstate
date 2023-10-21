use bs58::Alphabet;

use crate::utils;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct P2PKHDecoder {
    pub version: u8,
}

impl MulticoinDecoder for P2PKHDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let bytes_len = data.len();
        if bytes_len < 3 {
            return Err(MulticoinDecoderError::InvalidStructure("len < 3".to_string()));
        }

        if data[..2] != [0x76, 0xa9] {
            return Err(MulticoinDecoderError::InvalidStructure("invalid header".to_string()));
        }

        let len = data[2] as usize;
        let expected_len = 3 + len + 2;

        if bytes_len != expected_len {
            return Err(MulticoinDecoderError::InvalidStructure(format!(
                "invalid length ({bytes_len:?} != {expected_len:?})"
            )));
        }

        if data[bytes_len - 2..bytes_len] != [0x88, 0xac] {
            return Err(MulticoinDecoderError::InvalidStructure("invalid end".to_string()));
        }

        let pub_key_hash = &data[3..3 + len];

        let mut full = pub_key_hash.to_vec();
        full.insert(0, self.version);

        let full_checksum = utils::sha256::hash(utils::sha256::hash(full.clone()));

        full.extend_from_slice(&full_checksum[..4]);

        let value = bs58::encode(full)
            .with_alphabet(Alphabet::BITCOIN)
            .into_string();

        Ok(value)
    }
}

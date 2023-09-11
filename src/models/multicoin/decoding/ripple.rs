use bs58::Alphabet;

use crate::utils;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct RippleDecoder {}

// rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz
impl MulticoinDecoder for RippleDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let checksum = &utils::sha256::hash(utils::sha256::hash(data))[..4];

        Ok(bs58::encode([data, checksum].concat())
            .with_alphabet(Alphabet::RIPPLE)
            .into_string())
    }
}

// TODO: tests

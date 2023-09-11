use bech32::{ToBase32, Variant};

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct BinanceDecoder {}

impl MulticoinDecoder for BinanceDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        bech32::encode("bnb", data.to_base32(), Variant::Bech32)
            .map_err(|_| MulticoinDecoderError::InvalidStructure("failed to bech32 encode".to_string()))
    }
}

// TODO: tests
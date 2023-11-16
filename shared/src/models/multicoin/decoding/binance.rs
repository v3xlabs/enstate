use bech32::primitives::hrp::Hrp;
use bech32::Bech32;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct BinanceDecoder {}

impl MulticoinDecoder for BinanceDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        bech32::encode::<Bech32>(Hrp::parse_unchecked("bnb"), data).map_err(|_| {
            MulticoinDecoderError::InvalidStructure("failed to bech32 encode".to_string())
        })
    }
}

// TODO: tests

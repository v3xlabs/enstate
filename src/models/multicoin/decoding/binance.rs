use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct BinanceDecoder {}

impl MulticoinDecoder for BinanceDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests
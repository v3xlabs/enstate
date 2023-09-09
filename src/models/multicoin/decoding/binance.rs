use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct BinanceDecoder {}

impl MulticoinDecoder for BinanceDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        _ = data;
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests
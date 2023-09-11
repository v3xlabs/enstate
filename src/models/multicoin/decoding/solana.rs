use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct SolanaDecoder {}

impl MulticoinDecoder for SolanaDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        Ok(bs58::encode(data).into_string())
    }
}

// TODO: tests

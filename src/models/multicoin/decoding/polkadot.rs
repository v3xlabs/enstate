use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct PolkadotDecoder {}

impl MulticoinDecoder for PolkadotDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        _ = data;
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests

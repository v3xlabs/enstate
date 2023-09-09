use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct CardanoDecoder {}

impl MulticoinDecoder for CardanoDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        _ = data;
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests

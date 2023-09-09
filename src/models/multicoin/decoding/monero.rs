use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct MoneroDecoder {}

impl MulticoinDecoder for MoneroDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct RippleDecoder {}

impl MulticoinDecoder for RippleDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        _ = data;
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct TezosDecoder {}

impl MulticoinDecoder for TezosDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        _ = data;
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests

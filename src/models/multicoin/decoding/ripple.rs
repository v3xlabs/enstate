use super::{p2pkh::P2PKHDecoder, p2sh::P2SHDecoder, MulticoinDecoder, MulticoinDecoderError};

pub struct RippleDecoder {}

impl MulticoinDecoder for RippleDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests

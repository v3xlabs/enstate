use super::{MulticoinDecoder, MulticoinDecoderError, p2pkh::P2PKHDecoder, p2sh::P2SHDecoder};

pub struct PolkadotDecoder {}

impl MulticoinDecoder for PolkadotDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        Err(MulticoinDecoderError::NotSupported)
    }
}

// TODO: tests

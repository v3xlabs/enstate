use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct EvmDecoder {}

impl MulticoinDecoder for EvmDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let hex = hex::encode(data);

        Ok(format!("0x{}", hex))
    }
}

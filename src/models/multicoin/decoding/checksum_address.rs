use crate::utils;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct EvmDecoder {}

impl MulticoinDecoder for EvmDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let hex = utils::hex::encode_eip55(data);

        Ok(format!("0x{hex}"))
    }
}

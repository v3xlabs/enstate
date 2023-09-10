use crate::utils;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct EvmDecoder {
    pub(crate) chain_id: Option<u64>
}

impl MulticoinDecoder for EvmDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let hex = utils::hex::encode_rskip60(data, self.chain_id);

        Ok(format!("0x{hex}"))
    }
}

use crate::utils;
use crate::utils::eip55::RSKIPChain;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct EvmDecoder {
    pub(crate) chain: RSKIPChain
}

impl MulticoinDecoder for EvmDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let hex = utils::eip55::encode_rskip60(data, self.chain);

        Ok(format!("0x{hex}"))
    }
}

use bech32::Fe32;
use bech32::primitives::hrp::Hrp;

use crate::models::multicoin::decoding::{MulticoinDecoder, MulticoinDecoderError};

pub struct SegWitDecoder {
    pub human_readable_part: String,
}

impl MulticoinDecoder for SegWitDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        if data.len() < 2 {
            return Err(MulticoinDecoderError::InvalidStructure("len < 2".to_string()));
        }

        let version = match data[0] {
            0x00 => Ok(0x00),
            0x51..=0x60 => Ok(data[0] - 0x50),
            _ => Err(MulticoinDecoderError::InvalidStructure("invalid segwit version".to_string()))
        }?;

        bech32::segwit::encode(
            &Hrp::parse_unchecked(self.human_readable_part.as_str()),
            Fe32::try_from(version).unwrap(),
            &data[2..],
        )
            .map_err(|_| MulticoinDecoderError::InvalidStructure("failed to bech32 encode".to_string()))
    }
}

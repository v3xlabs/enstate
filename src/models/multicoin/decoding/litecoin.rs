use super::{MulticoinDecoder, MulticoinDecoderError, p2pkh::P2PKHDecoder, p2sh::P2SHDecoder};

pub struct LitecoinDecoder {}

impl MulticoinDecoder for LitecoinDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        if data.len() == 25 {
            return P2PKHDecoder { version: 0x30 }.decode(data);
        }

        if data.len() == 23 {
            return P2SHDecoder { version: 0x32 }.decode(data);
        }

        // ltc
        if data.starts_with(&[0x6c, 0x74, 0x63]) {
            return Err(MulticoinDecoderError::NotSupported);
        }

        Err(MulticoinDecoderError::InvalidStructure(String::new()))
    }
}

// TODO: tests
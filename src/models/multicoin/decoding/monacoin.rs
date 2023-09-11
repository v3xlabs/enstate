use super::{MulticoinDecoder, MulticoinDecoderError, p2pkh::P2PKHDecoder, p2sh::P2SHDecoder};

pub struct MonacoinDecoder {}

impl MulticoinDecoder for MonacoinDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        match data.len() {
            25 => P2PKHDecoder { version: 0x32 }.decode(data),
            23 => P2SHDecoder { version: 0x05 }.decode(data),
            _ => Err(MulticoinDecoderError::InvalidStructure(String::new()))
        }
    }
}

// TODO: tests

use super::{MulticoinDecoder, MulticoinDecoderError, p2pkh::P2PKHDecoder, p2sh::P2SHDecoder};

pub struct BitcoinCashDecoder {}

impl MulticoinDecoder for BitcoinCashDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        if data.len() == 25 {
            return P2PKHDecoder { version: 0x00 }.decode(data);
        }

        if data.len() == 23 {
            return P2SHDecoder { version: 0x05 }.decode(data);
        }

        Err(MulticoinDecoderError::InvalidStructure(String::new()))
    }
}

// TODO: tests

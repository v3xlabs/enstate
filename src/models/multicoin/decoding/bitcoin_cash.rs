use lazy_static::lazy_static;

use super::{MulticoinDecoder, MulticoinDecoderError, p2pkh::P2PKHDecoder, p2sh::P2SHDecoder};

pub struct BitcoinCashDecoder {}

lazy_static! {
    static ref BITCOIN_CASH_BYTES: &'static [u8] = "bitcoincash".as_bytes();
}

impl MulticoinDecoder for BitcoinCashDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        if data.len() == 25 {
            return P2PKHDecoder { version: 0x00 }.decode(data);
        }

        if data.len() == 23 {
            return P2SHDecoder { version: 0x05 }.decode(data);
        }

        if data.starts_with(*BITCOIN_CASH_BYTES) {
            return Err(MulticoinDecoderError::NotSupported);
        }

        Err(MulticoinDecoderError::InvalidStructure(String::new()))
    }
}

// TODO: tests

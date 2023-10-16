use base32::Alphabet;
use crc16::{State, XMODEM};

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct StellarDecoder {}

impl MulticoinDecoder for StellarDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        // starts with ed25519 version byte (6 << 3)
        let mut full: Vec<u8> = vec![0x30];
        full.extend_from_slice(data);

        let checksum = State::<XMODEM>::calculate(full.as_slice());

        full.push((checksum & 0xff) as u8);
        full.push(((checksum >> 8) & 0xff) as u8);

        Ok(base32::encode(Alphabet::RFC4648 { padding: false }, full.as_slice()))
    }
}

// TODO: tests

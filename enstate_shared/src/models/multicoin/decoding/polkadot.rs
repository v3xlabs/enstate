use blake2::{Blake2b512, Digest};
use bs58::Alphabet;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct PolkadotDecoder {}

impl MulticoinDecoder for PolkadotDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let mut hasher = Blake2b512::new();
        hasher.update([b"SS58PRE" as &[u8], &[0], data].concat());

        let hash = hasher.finalize();

        Ok(bs58::encode([&[0], data, &hash.as_slice()[..2]].concat())
            .with_alphabet(Alphabet::BITCOIN)
            .into_string())
    }
}

// TODO: tests

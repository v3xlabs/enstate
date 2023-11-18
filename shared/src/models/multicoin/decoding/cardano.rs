use bech32::primitives::hrp::Hrp;
use bech32::Bech32;
use bs58::Alphabet;
use ciborium::value::Integer;
use ciborium::Value;

use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct CardanoDecoder {}

// None if invalid bryon address
fn encode_cardano_bryon(data: &[u8]) -> Result<String, MulticoinDecoderError> {
    let checksum = crc32fast::hash(data);
    let mut cbor_encoded: Vec<u8> = Vec::new();

    ciborium::into_writer(
        &vec![
            Value::Tag(24, Box::new(Value::Bytes(data.to_vec()))),
            Value::Integer(Integer::from(checksum)),
        ],
        &mut cbor_encoded,
    )
    .map_err(|_| MulticoinDecoderError::InvalidStructure("failed to cbor encode".to_string()))?;

    let bs58_encoded = bs58::encode(cbor_encoded)
        .with_alphabet(Alphabet::BITCOIN)
        .into_string();

    if !bs58_encoded.starts_with("Ae2") && !bs58_encoded.starts_with("Ddz") {
        return Err(MulticoinDecoderError::InvalidStructure(
            "invalid bryon address prefix".to_string(),
        ));
    }

    Ok(bs58_encoded)
}

fn encode_cardano_shelley(data: &[u8]) -> Result<String, MulticoinDecoderError> {
    bech32::encode::<Bech32>(Hrp::parse_unchecked("addr"), data)
        .map_err(|_| MulticoinDecoderError::InvalidStructure("failed to bech32 encode".to_string()))
}

impl MulticoinDecoder for CardanoDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        encode_cardano_bryon(data).or_else(|_| encode_cardano_shelley(data))
    }
}

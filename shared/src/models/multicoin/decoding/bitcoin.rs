use std::string::ToString;

use lazy_static::lazy_static;

use crate::models::multicoin::decoding::segwit::SegWitDecoder;

use super::{p2pkh::P2PKHDecoder, p2sh::P2SHDecoder, MulticoinDecoder, MulticoinDecoderError};

lazy_static! {
    static ref BTC_SEGWIT_DECODER: SegWitDecoder = SegWitDecoder {
        human_readable_part: "bc".to_string()
    };
}

pub struct BitcoinDecoder {}

impl MulticoinDecoder for BitcoinDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        if let Ok(address) = BTC_SEGWIT_DECODER.decode(data) {
            return Ok(address);
        }

        if data.len() == 25 {
            return P2PKHDecoder { version: 0x00 }.decode(data);
        }

        if data.len() == 23 {
            return P2SHDecoder { version: 0x05 }.decode(data);
        }

        Err(MulticoinDecoderError::InvalidStructure(String::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_btc_p2pkh() {
        let decoded = BitcoinDecoder {}
            .decode(&hex_literal::hex!(
                "76a91462e907b15cbf27d5425399ebf6f0fb50ebb88f1888ac"
            ))
            .unwrap();

        assert_eq!(decoded, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string());
    }

    #[tokio::test]
    async fn test_btc_p2sh() {
        let decoded = BitcoinDecoder {}
            .decode(&hex_literal::hex!(
                "a91462e907b15cbf27d5425399ebf6f0fb50ebb88f1887"
            ))
            .unwrap();

        assert_eq!(decoded, "3Ai1JZ8pdJb2ksieUV8FsxSNVJCpoPi8W6".to_string());
    }

    #[tokio::test]
    async fn test_btc_segwit() {
        let decoded = BitcoinDecoder {}
            .decode(&hex_literal::hex!(
                "0014751e76e8199196d454941c45d1b3a323f1433bd6"
            ))
            .unwrap();

        assert_eq!(
            decoded,
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4".to_string()
        );
    }
}

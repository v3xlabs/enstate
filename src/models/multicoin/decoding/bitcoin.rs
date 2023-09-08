use thiserror::Error;

use super::{p2pkh, p2sh};

#[derive(Error, Debug)]
pub enum BtcDecodeError {
    #[error("Invalid address")]
    InvalidAddress,

    #[error("Address type not supported")]
    NotSupported
}

pub fn decode_btc(bytes: &[u8]) -> Result<String, BtcDecodeError> {
    if bytes.len() == 25 {
        return p2pkh::decode(bytes, 0x00).map_err(|_| BtcDecodeError::InvalidAddress)
    }

    if bytes.len() == 23 {
        return p2sh::decode(bytes, 0x05).map_err(|_| BtcDecodeError::InvalidAddress)
    }

    if bytes.starts_with(&[0x98, 0x99]) {
        return Err(BtcDecodeError::NotSupported)
    }

    Err(BtcDecodeError::NotSupported)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_btc_p2pkh() {
        let decoded = decode_btc(
            &hex::decode("76a91462e907b15cbf27d5425399ebf6f0fb50ebb88f1888ac").unwrap()
        ).unwrap();

        assert_eq!(decoded, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string());
    }

    #[tokio::test]
    async fn test_btc_p2sh() {
        let decoded = decode_btc(
            &hex::decode("a91462e907b15cbf27d5425399ebf6f0fb50ebb88f1887").unwrap()
        ).unwrap();

        assert_eq!(decoded, "3Ai1JZ8pdJb2ksieUV8FsxSNVJCpoPi8W6".to_string());
    }

    #[tokio::test]
    async fn test_btc_segwit() {
        let decoded = decode_btc(
            &hex::decode("0014751e76e8199196d454941c45d1b3a323f1433bd6").unwrap()
        ).unwrap();

        assert_eq!(decoded, "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4".to_string());
    }
}

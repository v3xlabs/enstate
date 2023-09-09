use thiserror::Error;

use self::{bitcoin::BitcoinDecoder, checksum_address::EvmDecoder};

use super::cointype::{coins::CoinType, slip44::SLIP44};

pub mod bitcoin;
pub mod checksum_address;
pub mod p2pkh;
pub mod p2sh;

#[derive(Debug, Error)]
pub enum MulticoinDecoderError {
    #[error("Invalid Structure {0}")]
    InvalidStructure(String),

    #[error("Not supported")]
    NotSupported,
}

pub trait MulticoinDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError>;
}

impl CoinType {
    pub fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        let decoder: Box<dyn MulticoinDecoder> = match self {
            Self::Slip44(slip44) => match slip44 {
                SLIP44::Bitcoin => Box::new(BitcoinDecoder {}),
                _ => return Err(MulticoinDecoderError::NotSupported),
            },
            Self::Evm(chain) => Box::new(EvmDecoder {}),
        };

        decoder.decode(data)
    }
}

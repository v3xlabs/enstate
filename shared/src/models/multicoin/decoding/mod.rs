use thiserror::Error;

use crate::models::multicoin::decoding::binance::BinanceDecoder;
use crate::models::multicoin::decoding::bitcoin_cash::BitcoinCashDecoder;
use crate::models::multicoin::decoding::cardano::CardanoDecoder;
use crate::models::multicoin::decoding::dogecoin::DogecoinDecoder;
use crate::models::multicoin::decoding::hedera::HederaDecoder;
use crate::models::multicoin::decoding::litecoin::LitecoinDecoder;
use crate::models::multicoin::decoding::monacoin::MonacoinDecoder;
use crate::models::multicoin::decoding::monero::MoneroDecoder;
use crate::models::multicoin::decoding::polkadot::PolkadotDecoder;
use crate::models::multicoin::decoding::ripple::RippleDecoder;
use crate::models::multicoin::decoding::solana::SolanaDecoder;
use crate::models::multicoin::decoding::stellar::StellarDecoder;
use crate::models::multicoin::decoding::tezos::TezosDecoder;
use crate::utils::eip55::RSKIPChain;

use super::cointype::{coins::CoinType, slip44::SLIP44};

use self::{bitcoin::BitcoinDecoder, checksum_address::EvmDecoder};

pub mod binance;
pub mod bitcoin;
pub mod bitcoin_cash;
pub mod cardano;
pub mod checksum_address;
pub mod dogecoin;
pub mod hedera;
pub mod litecoin;
pub mod monacoin;
pub mod monero;
pub mod p2pkh;
pub mod p2sh;
pub mod polkadot;
pub mod ripple;
pub mod segwit;
pub mod solana;
pub mod stellar;
pub mod tezos;

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
                SLIP44::Ethereum | SLIP44::EthereumClassic => Box::new(EvmDecoder {
                    chain: RSKIPChain::Ethereum,
                }),
                SLIP44::Rootstock => Box::new(EvmDecoder {
                    chain: RSKIPChain::Other(30),
                }),
                SLIP44::Litecoin => Box::new(LitecoinDecoder {}),
                SLIP44::BitcoinCash => Box::new(BitcoinCashDecoder {}),
                SLIP44::Solana => Box::new(SolanaDecoder {}),
                SLIP44::Hedera => Box::new(HederaDecoder {}),
                SLIP44::Stellar => Box::new(StellarDecoder {}),
                SLIP44::Dogecoin => Box::new(DogecoinDecoder {}),
                SLIP44::Monacoin => Box::new(MonacoinDecoder {}),
                SLIP44::Monero => Box::new(MoneroDecoder {}),
                SLIP44::Ripple => Box::new(RippleDecoder {}),
                SLIP44::Tezos => Box::new(TezosDecoder {}),
                SLIP44::Cardano => Box::new(CardanoDecoder {}),
                SLIP44::Binance => Box::new(BinanceDecoder {}),
                SLIP44::Polkadot => Box::new(PolkadotDecoder {}),
                _ => return Err(MulticoinDecoderError::NotSupported),
            },
            Self::Evm(id) => Box::new(EvmDecoder {
                chain: RSKIPChain::Other(id.as_chain_id()),
            }),
        };

        decoder.decode(data)
    }
}

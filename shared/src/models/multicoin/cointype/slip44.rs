use std::fmt::{Display, Formatter};

use ethers_core::types::U256;

use super::CoinType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SLIP44 {
    Tezos,
    Hedera,
    Monero,
    Ripple,
    Solana,
    Cardano,
    Stellar,
    Bitcoin,
    Binance,
    Litecoin,
    Dogecoin,
    Ethereum,
    Monacoin,
    Polkadot,
    Rootstock,
    BitcoinCash,
    EthereumClassic,
    Other(U256),
}

impl From<SLIP44> for U256 {
    fn from(val: SLIP44) -> Self {
        match val {
            SLIP44::Bitcoin => Self::from(0),
            SLIP44::Litecoin => Self::from(2),
            SLIP44::Dogecoin => Self::from(3),
            SLIP44::Ethereum => Self::from(60),
            SLIP44::BitcoinCash => Self::from(145),
            SLIP44::EthereumClassic => Self::from(61),
            SLIP44::Monero => Self::from(128),
            SLIP44::Ripple => Self::from(144),
            SLIP44::Stellar => Self::from(148),
            SLIP44::Tezos => Self::from(1729),
            SLIP44::Hedera => Self::from(3030),
            SLIP44::Cardano => Self::from(1815),
            SLIP44::Rootstock => Self::from(137),
            SLIP44::Monacoin => Self::from(22),
            SLIP44::Binance => Self::from(714),
            SLIP44::Solana => Self::from(501),
            SLIP44::Polkadot => Self::from(354),
            SLIP44::Other(u256) => u256,
        }
    }
}

impl From<SLIP44> for CoinType {
    fn from(val: SLIP44) -> Self {
        CoinType::Slip44(val)
    }
}

impl Display for SLIP44 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let coin_name = match self {
            Self::Bitcoin => "btc".to_string(),
            Self::Litecoin => "ltc".to_string(),
            Self::Dogecoin => "doge".to_string(),
            Self::Ethereum => "eth".to_string(),
            Self::BitcoinCash => "bch".to_string(),
            Self::EthereumClassic => "etc".to_string(),
            Self::Monero => "xmr".to_string(),
            Self::Ripple => "ripple".to_string(),
            Self::Stellar => "stellar".to_string(),
            Self::Tezos => "tezos".to_string(),
            Self::Hedera => "hedera".to_string(),
            Self::Cardano => "cardano".to_string(),
            Self::Monacoin => "mona".to_string(),
            Self::Rootstock => "rootstock".to_string(),
            Self::Binance => "bnb".to_string(),
            Self::Solana => "sol".to_string(),
            Self::Polkadot => "dot".to_string(),
            Self::Other(u256) => u256.to_string(),
        };

        f.write_str(coin_name.as_str())
    }
}

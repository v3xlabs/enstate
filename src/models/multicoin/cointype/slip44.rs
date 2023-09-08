use ethers_core::types::U256;

use super::CoinType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SLIP44 {
    Tezos,
    Hedera,
    Monero,
    Ripple,
    Cardano,
    Stellar,
    Bitcoin,
    Litecoin,
    Dogecoin,
    Ethereum,
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
            SLIP44::Other(u256) => u256,
        }
    }
}

impl From<SLIP44> for CoinType {
    fn from(val: SLIP44) -> Self {
        CoinType::Slip44(val)
    }
}

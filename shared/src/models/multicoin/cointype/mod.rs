use self::{coins::CoinType, evm::ChainId, slip44::SLIP44};

pub mod coins;
pub mod evm;
pub mod slip44;

pub struct Coins {
    pub coins: Vec<CoinType>,
}

impl Default for Coins {
    fn default() -> Self {
        Self {
            coins: vec![
                SLIP44::Tezos.into(),
                SLIP44::Hedera.into(),
                SLIP44::Monero.into(),
                SLIP44::Ripple.into(),
                SLIP44::Solana.into(),
                SLIP44::Cardano.into(),
                SLIP44::Stellar.into(),
                SLIP44::Bitcoin.into(),
                SLIP44::Binance.into(),
                SLIP44::Litecoin.into(),
                SLIP44::Dogecoin.into(),
                SLIP44::Ethereum.into(),
                SLIP44::Monacoin.into(),
                SLIP44::Polkadot.into(),
                SLIP44::Rootstock.into(),
                SLIP44::BitcoinCash.into(),
                SLIP44::EthereumClassic.into(),
                ChainId::Ethereum.into(),
                ChainId::Polygon.into(),
                ChainId::Optimism.into(),
                ChainId::Arbitrum.into(),
                ChainId::Gnosis.into(),
                ChainId::BinanceSmartChain.into(),
                ChainId::Avalanche.into(),
                ChainId::Fantom.into(),
                ChainId::Celo.into(),
                ChainId::Moonbeam.into(),
            ],
        }
    }
}

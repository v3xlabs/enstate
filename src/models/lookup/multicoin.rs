use ethers_core::{
    abi::{ParamType, Token},
    types::{H160, H256},
};
use hex_literal::hex;
use tracing::info;

use crate::models::multicoin::cointype::{coins::CoinType, evm::ChainId, slip44::SLIP44};

use super::{ENSLookup, ENSLookupError};

pub struct Multicoin {
    pub coin_type: CoinType,
}

impl ENSLookup for Multicoin {
    fn calldata(&self, namehash: &H256) -> Vec<u8> {
        let fn_selector = hex!("f1cb7e06").to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::Uint(self.coin_type.clone().into()),
        ]);

        [fn_selector, data].concat()
    }

    fn decode(&self, data: &[u8]) -> Result<Option<String>, ENSLookupError> {
        info!("Decoding: {:?}", data);

        let decoded_abi = ethers_core::abi::decode(&[ParamType::Bytes], data)
            .map_err(|_| ENSLookupError::AbiError)?;
        let value = decoded_abi
            .get(0)
            .ok_or(ENSLookupError::AbiError)?
            .clone()
            .into_bytes();

        if value.is_none() {
            return Ok(None);
        }

        let value = value.unwrap();

        if value.is_empty() {
            return Ok(None);
        }

        match &self.coin_type {
            // SLIP-044 Chain Address Decoding (see ensip-9)
            CoinType::Slip44(slip44) => match slip44 {
                // Bitcoin Decoding
                SLIP44::Bitcoin => Ok(Some(format!("btc:{}", bs58::encode(value).into_string()))),
                // Lightcoin Decoding
                SLIP44::Litecoin => Ok(Some(format!("ltc:{}", bs58::encode(value).into_string()))),

                // Unsupported SLIP44 Chain
                _ => Ok(Some(format!("SLIP-{:?}", value))),
            },
            // Implement EVM Chain Address Decoding (mostly ChecksummedHex, sometimes ChecksummedHex(chainId)) (see ensip-11)
            CoinType::Evm(evm) => match evm {
                // TODO: EVM Exceptions go here
                // ChainId::Ethereum => Ok(value.map(|value| format!("EVM-ETH-{:?}", value))),

                // Every EVM Chain
                _ => {
                    // Verify length is 20 bytes
                    if value.len() != 20 {
                        // TODO: throw invalid length
                        return Ok(Some("Invalid Length".to_string()));
                    }

                    let address = hex::encode(value);

                    Ok(Some(format!("0x{address}")))
                }
            },
        }
    }
}

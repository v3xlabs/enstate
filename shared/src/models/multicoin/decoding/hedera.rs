use super::{MulticoinDecoder, MulticoinDecoderError};

pub struct HederaDecoder {}

impl MulticoinDecoder for HederaDecoder {
    fn decode(&self, data: &[u8]) -> Result<String, MulticoinDecoderError> {
        if data.len() != 20 {
            return Err(MulticoinDecoderError::InvalidStructure(String::new()));
        }

        let shard = u32::from_be_bytes((&data[..4]).try_into().unwrap());
        let realm = u64::from_be_bytes((&data[4..12]).try_into().unwrap());
        let account = u64::from_be_bytes((&data[12..]).try_into().unwrap());

        Ok(format!("{shard}.{realm}.{account}"))
    }
}

// TODO: tests

pub fn encode_rskip60(data: &[u8], chain_id: Option<u64>) -> String {
    let raw = hex::encode(data).to_ascii_lowercase();
    let hash = ethers::utils::keccak256(
    format!("{:}{raw}",
            if chain_id.is_none() {
                String::new()
            } else {
                format!("{:}0x", chain_id.unwrap())
            }
        )
    );

    return raw.chars().enumerate()
        .map(|(i, c)|
            if hash[i >> 1] << ((i & 1) << 2) >= 0x80 {
                c.to_ascii_uppercase()
            } else {
                c
            }
        )
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rskip60_eth() {
        assert_eq!(
            encode_rskip60(hex::decode("2b5c7025998f88550ef2fece8bf87935f542c190").unwrap().as_slice(), None),
            "2B5c7025998f88550Ef2fEce8bf87935f542C190".to_string()
        );

        assert_eq!(
            encode_rskip60(hex::decode("77c0f878fde3d1ad2830ca9e46d7c47c951a93fb").unwrap().as_slice(), None),
            "77c0F878FdE3d1ad2830ca9E46D7c47C951A93fb".to_string()
        );
    }

    #[tokio::test]
    async fn test_rskip60_rootstock() {
        assert_eq!(
            encode_rskip60(hex::decode("5aaeb6053f3e94c9b9a09f33669435e7ef1beaed").unwrap().as_slice(), Some(30)),
            "5aaEB6053f3e94c9b9a09f33669435E7ef1bEAeD".to_string()
        );

        assert_eq!(
            encode_rskip60(hex::decode("fb6916095ca1df60bb79ce92ce3ea74c37c5d359").unwrap().as_slice(), Some(30)),
            "Fb6916095cA1Df60bb79ce92cE3EA74c37c5d359".to_string()
        );
    }
}
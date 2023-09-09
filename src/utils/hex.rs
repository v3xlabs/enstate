pub fn encode_eip55(data: &[u8]) -> String {
    let raw = hex::encode(data).to_ascii_lowercase();
    let hash = ethers::utils::keccak256(raw.as_bytes());

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
    async fn test_eip55() {
        assert_eq!(
            encode_eip55(hex::decode("2b5c7025998f88550ef2fece8bf87935f542c190").unwrap().as_slice()),
            "2B5c7025998f88550Ef2fEce8bf87935f542C190".to_string()
        );

        assert_eq!(
            encode_eip55(hex::decode("77c0f878fde3d1ad2830ca9e46d7c47c951a93fb").unwrap().as_slice()),
            "77c0F878FdE3d1ad2830ca9E46D7c47C951A93fb".to_string()
        );
    }
}
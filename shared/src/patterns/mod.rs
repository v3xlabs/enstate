use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Not exact but rough enough for now.
    static ref DOMAIN_REGEX: Regex = Regex::new(r"^(?:[^.]+\.)+[a-zA-Z]{2,}$").unwrap();

    static ref ADDRESS_REGEX: Regex = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
}

pub fn test_domain(raw: &str) -> bool {
    DOMAIN_REGEX.is_match(raw)
}

pub fn test_address(raw: &str) -> bool {
    ADDRESS_REGEX.is_match(raw)
}

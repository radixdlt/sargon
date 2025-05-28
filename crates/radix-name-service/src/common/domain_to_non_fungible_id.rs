use crate::prelude::*;
use std::str::FromStr;

use addresses::NonFungibleLocalId;
use bytes::CommonError;
use k256::sha2::{Digest, Sha256};

pub fn domain_to_non_fungible_id(
    domain: &str,
    is_byte_id: bool,
) -> Result<NonFungibleLocalId> {
    if !domain.is_ascii() {
        return Err(CommonError::Unknown);
    }

    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    let hash = hasher.finalize();

    let truncated_hash = &hash[..16];
    let hex_string: String = truncated_hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .rev()
        .collect();

    let id = if is_byte_id {
        format!("[{}]", hex_string)
    } else {
        hex_string
    };

    NonFungibleLocalId::from_str(&id)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_domain_to_non_fungible_id() {
        let domain = "bakirci.xrd";
        let expected_hex = "9a5fb8db4539384dfe275647bfef559e";
        let expected_id = NonFungibleLocalId::from_str(expected_hex).unwrap();
        let expected_byte_id =
            NonFungibleLocalId::from_str(&format!("[{}]", expected_hex))
                .unwrap();

        assert_eq!(
            super::domain_to_non_fungible_id(domain, false).unwrap(),
            expected_id
        );
        assert_eq!(
            super::domain_to_non_fungible_id(domain, true).unwrap(),
            expected_byte_id
        );
    }
}

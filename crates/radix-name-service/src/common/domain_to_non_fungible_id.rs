use crate::prelude::*;
use k256::sha2::{Digest, Sha256};

/// Converts a domain string to a NonFungibleLocalId by hashing the domain.
/// There is deterministic mapping from domain to NonFungibleLocalId.
pub fn domain_to_non_fungible_id(domain: &str) -> Result<NonFungibleLocalId> {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    let hash = hasher.finalize();

    let truncated_hash = &hash[..16];
    let hex_string: String = truncated_hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .rev()
        .collect();

    let id = format!("[{}]", hex_string);

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
            super::domain_to_non_fungible_id(domain).unwrap(),
            expected_id
        );
        assert_eq!(
            super::domain_to_non_fungible_id(domain).unwrap(),
            expected_byte_id
        );
    }
}

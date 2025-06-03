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
        let expected_hex = "[99a979b2006cead35c3c9209ff7d1c63]";
        let expected_id = NonFungibleLocalId::from_str(expected_hex).unwrap();

        assert_eq!(
            super::domain_to_non_fungible_id(domain).unwrap(),
            expected_id
        );
    }
}

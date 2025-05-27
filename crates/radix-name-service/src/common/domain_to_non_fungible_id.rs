use k256::sha2::{Digest, Sha256};

pub fn domain_to_non_fungible_id(
    domain: &str,
    is_byte_id: bool,
) -> Result<String, String> {
    if !domain.is_ascii() {
        return Err("Domain must be ASCII".to_string());
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

    if is_byte_id {
        Ok(format!("[{}]", hex_string))
    } else {
        Ok(hex_string)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_domain_to_non_fungible_id() {
        let domain = "bakirci.xrd";
        let expected_hex = "9a5fb8db4539384dfe275647bfef559e";
        let expected_byte_id = format!("[{}]", expected_hex);

        assert_eq!(
            super::domain_to_non_fungible_id(domain, false).unwrap(),
            expected_hex
        );
        assert_eq!(
            super::domain_to_non_fungible_id(domain, true).unwrap(),
            expected_byte_id
        );
    }
}
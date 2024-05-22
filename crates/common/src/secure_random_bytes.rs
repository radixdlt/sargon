use rand::{rngs::OsRng, RngCore};

/// Generates `N` random bytes using a cryptographically
/// secure random generator and returns these bytes as
/// a Vec<u8>.
pub fn generate_byte_array<const N: usize>() -> [u8; N] {
    let mut csprng = OsRng;
    let mut bytes: [u8; N] = [0u8; N];
    csprng.fill_bytes(&mut bytes);
    bytes
}

/// Generates `N` random bytes using a cryptographically
/// secure random generator and returns these bytes as
/// a Vec<u8>.
pub fn generate_bytes<const N: usize>() -> Vec<u8> {
    generate_byte_array::<N>().to_vec()
}

/// Generates `32` random bytes using a cryptographically
/// secure random generator and returns these bytes as
/// a Vec<u8>.
pub fn generate_32_bytes() -> Vec<u8> {
    generate_bytes::<32>()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::generate_32_bytes;

    #[test]
    fn random() {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            let bytes = generate_32_bytes();
            assert_eq!(bytes.len(), 32);
            set.insert(bytes);
        }
        assert_eq!(set.len(), n);
    }
}

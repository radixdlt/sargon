use crate::prelude::*;

#[uniffi::export]
pub fn hash_get_bytes(hash: &Hash) -> BagOfBytes {
    BagOfBytes::from(hash.bytes())
}

#[uniffi::export]
pub fn new_hash_sample() -> Hash {
    Hash::sample()
}

#[uniffi::export]
pub fn new_hash_sample_other() -> Hash {
    Hash::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Hash;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_hash_sample(),
                new_hash_sample_other(),
                // duplicates should get removed
                new_hash_sample(),
                new_hash_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_hash_get_bytes() {
        assert_eq!(
            hex_encode(hash_get_bytes(&SUT::sample())),
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
        );
    }
}

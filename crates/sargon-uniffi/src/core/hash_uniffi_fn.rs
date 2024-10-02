use crate::prelude::*;

#[uniffi::export]
pub fn hash_get_bytes(hash: &Hash) -> Exactly32Bytes {
    Exactly32Bytes::from(*hash)
}

#[uniffi::export]
pub fn new_hash_sample() -> Hash {
    Hash::sample()
}

#[uniffi::export]
pub fn new_hash_sample_other() -> Hash {
    Hash::sample_other()
}

#[uniffi::export]
pub fn new_hash_from_bytes(bytes: Exactly32Bytes) -> Hash {
    Hash::from(bytes)
}

#[uniffi::export]
pub fn new_hash_from_string(string: String) -> Result<Hash> {
    Exactly32Bytes::from_str(&string).map(Hash::from)
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

    #[test]
    fn test_from_str() {
        let sut = new_hash_from_string(
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
                .to_owned(),
        )
        .unwrap();
        assert_eq!(sut, SUT::sample())
    }

    #[test]
    fn test_from_bytes() {
        let bytes = Exactly32Bytes::from_str(
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935",
        )
        .unwrap();
        let sut = new_hash_from_bytes(bytes);
        assert_eq!(sut, SUT::sample())
    }
}

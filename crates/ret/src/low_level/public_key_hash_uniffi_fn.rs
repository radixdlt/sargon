use crate::prelude::*;

#[uniffi::export]
pub fn new_public_key_hash_of_key(public_key: PublicKey) -> PublicKeyHash {
    PublicKeyHash::hash(public_key)
}

#[uniffi::export]
pub fn new_public_key_hash_sample() -> PublicKeyHash {
    PublicKeyHash::sample()
}

#[uniffi::export]
pub fn new_public_key_hash_sample_other() -> PublicKeyHash {
    PublicKeyHash::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PublicKeyHash;

    #[test]
    fn hash_of_key() {
        assert_eq!(
            new_public_key_hash_of_key(PublicKey::sample()),
            SUT::sample()
        );
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_public_key_hash_sample(),
                new_public_key_hash_sample_other(),
                // duplicates should get removed
                new_public_key_hash_sample(),
                new_public_key_hash_sample_other(),
            ])
            .len(),
            2
        );
    }
}

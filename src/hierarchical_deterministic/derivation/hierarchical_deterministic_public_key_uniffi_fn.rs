use crate::prelude::*;

#[uniffi::export]
pub fn new_hierarchical_deterministic_public_key_sample(
) -> HierarchicalDeterministicPublicKey {
    HierarchicalDeterministicPublicKey::sample()
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_public_key_sample_other(
) -> HierarchicalDeterministicPublicKey {
    HierarchicalDeterministicPublicKey::sample_other()
}
#[uniffi::export]
pub fn hierarchical_deterministic_public_key_is_valid_signature(
    key: &HierarchicalDeterministicPublicKey,
    signature: Signature,
    for_hash: &Hash,
) -> bool {
    key.is_valid(signature, for_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HierarchicalDeterministicPublicKey;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_hierarchical_deterministic_public_key_sample(),
                new_hierarchical_deterministic_public_key_sample_other(),
                // duplicates should get removed
                new_hierarchical_deterministic_public_key_sample(),
                new_hierarchical_deterministic_public_key_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn is_valid() {
        let private_key = HierarchicalDeterministicPrivateKey::sample();
        let msg = Hash::sample();
        let sut = private_key.sign(&msg);
        let public_key = private_key.public_key();
        assert!(hierarchical_deterministic_public_key_is_valid_signature(
            &public_key,
            sut.signature(),
            &msg
        ));
    }
}

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
}

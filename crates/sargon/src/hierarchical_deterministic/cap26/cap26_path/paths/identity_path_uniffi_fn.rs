use crate::prelude::*;

#[uniffi::export]
pub fn new_identity_path_sample() -> IdentityPath {
    IdentityPath::sample()
}

#[uniffi::export]
pub fn new_identity_path_sample_other() -> IdentityPath {
    IdentityPath::sample_other()
}

#[uniffi::export]
pub fn new_identity_path(
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: HDPathValue,
) -> IdentityPath {
    IdentityPath::new(network_id, key_kind, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentityPath;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_identity_path_sample(),
                new_identity_path_sample_other(),
                // duplicates should get removed
                new_identity_path_sample(),
                new_identity_path_sample_other(),
            ])
            .len(),
            2
        );
    }
}

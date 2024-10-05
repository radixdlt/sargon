use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{}", self.bip32_string())]
pub struct IdentityPath {
    pub path: HDPath,

    pub network_id: NetworkID,

    pub entity_kind: CAP26EntityKind,

    pub key_kind: CAP26KeyKind,

    pub index: HDPathValue,
}

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

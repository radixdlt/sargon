use crate::prelude::*;

#[uniffi::export]
pub fn new_account_path_sample() -> AccountPath {
    AccountPath::sample()
}

#[uniffi::export]
pub fn new_account_path_sample_other() -> AccountPath {
    AccountPath::sample_other()
}

#[uniffi::export]
pub fn new_account_path(
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: HDPathValue,
) -> AccountPath {
    AccountPath::new(network_id, key_kind, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountPath;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_path_sample(),
                new_account_path_sample_other(),
                // duplicates should get removed
                new_account_path_sample(),
                new_account_path_sample_other(),
            ])
            .len(),
            2
        );
    }
}

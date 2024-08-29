use crate::prelude::*;

#[uniffi::export]
pub fn new_authorized_dapp_detailed_sample() -> AuthorizedDappDetailed {
    AuthorizedDappDetailed::sample()
}

#[uniffi::export]
pub fn new_authorized_dapp_detailed_sample_other() -> AuthorizedDappDetailed {
    AuthorizedDappDetailed::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedDappDetailed;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_authorized_dapp_detailed_sample(),
                new_authorized_dapp_detailed_sample_other(),
                // duplicates should get removed
                new_authorized_dapp_detailed_sample(),
                new_authorized_dapp_detailed_sample_other(),
            ])
            .len(),
            2
        );
    }
}

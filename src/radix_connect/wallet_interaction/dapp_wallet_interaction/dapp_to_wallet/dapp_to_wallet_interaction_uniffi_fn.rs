use crate::prelude::*;

#[uniffi::export]
pub(crate) fn new_dapp_to_wallet_interaction_unvalidated_sample(
) -> DappToWalletInteractionUnvalidated {
    DappToWalletInteractionUnvalidated::sample()
}

#[uniffi::export]
pub(crate) fn new_dapp_to_wallet_interaction_unvalidated_sample_other(
) -> DappToWalletInteractionUnvalidated {
    DappToWalletInteractionUnvalidated::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionUnvalidated;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_dapp_to_wallet_interaction_unvalidated_sample(),
                new_dapp_to_wallet_interaction_unvalidated_sample_other(),
                // duplicates should get removed
                new_dapp_to_wallet_interaction_unvalidated_sample(),
                new_dapp_to_wallet_interaction_unvalidated_sample_other(),
            ])
            .len(),
            2
        );
    }
}

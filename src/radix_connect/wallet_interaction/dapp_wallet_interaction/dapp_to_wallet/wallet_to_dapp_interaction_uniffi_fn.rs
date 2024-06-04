use crate::prelude::*;

#[uniffi::export]
pub(crate) fn new_wallet_to_dapp_interaction_response_sample(
) -> WalletToDappInteractionResponse {
    WalletToDappInteractionResponse::sample()
}

#[uniffi::export]
pub(crate) fn new_wallet_to_dapp_interaction_response_sample_other(
) -> WalletToDappInteractionResponse {
    WalletToDappInteractionResponse::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionResponse;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_wallet_to_dapp_interaction_response_sample(),
                new_wallet_to_dapp_interaction_response_sample_other(),
                // duplicates should get removed
                new_wallet_to_dapp_interaction_response_sample(),
                new_wallet_to_dapp_interaction_response_sample_other(),
            ])
            .len(),
            2
        );
    }
}

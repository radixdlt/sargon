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

    #[test]
    fn inequality_of_samples() {
        assert_ne!(
            new_wallet_to_dapp_interaction_response_sample(),
            new_wallet_to_dapp_interaction_response_sample_other()
        );
    }
}

use crate::prelude::*;

json_data_convertible!(DappToWalletInteractionUnvalidated);
json_data_convertible!(WalletToDappInteractionResponse);

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

    #[test]
    fn inequality_of_samples() {
        assert_ne!(
            new_dapp_to_wallet_interaction_unvalidated_sample(),
            new_dapp_to_wallet_interaction_unvalidated_sample_other()
        );
    }
}

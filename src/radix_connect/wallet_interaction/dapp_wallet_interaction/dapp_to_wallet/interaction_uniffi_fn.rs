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

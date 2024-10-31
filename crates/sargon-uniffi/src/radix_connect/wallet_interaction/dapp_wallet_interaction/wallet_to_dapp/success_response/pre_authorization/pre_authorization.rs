use crate::prelude::*;
use sargon::WalletToDappInteractionPreAuthorizationResponseItems as InternalWalletToDappInteractionPreAuthorizationResponseItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionPreAuthorizationResponseItems {
    /// A hex encoded signed partial transaction.
    pub encoded_signed_partial_transaction: String,
}

#[uniffi::export]
pub fn new_wallet_to_dapp_interaction_pre_authorization_response_items(
    signed_subintent: SignedSubintent,
) -> WalletToDappInteractionPreAuthorizationResponseItems {
    InternalWalletToDappInteractionPreAuthorizationResponseItems::new(
        signed_subintent.into(),
    )
    .into()
}

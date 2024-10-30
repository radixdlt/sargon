use crate::prelude::*;
use sargon::WalletToDappInteractionPreAuthorizationResponseItems as InternalWalletToDappInteractionPreAuthorizationResponseItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionPreAuthorizationResponseItems {
    /// A hex encoded signed partial transaction.
    pub encoded_signed_partial_transaction: String,
}

#[uniffi::export]
pub fn new_wallet_to_dapp_interaction_pre_authorization_response_items(
    subintent: Subintent,
    signatures: Vec<IntentSignature>,
) -> Result<WalletToDappInteractionPreAuthorizationResponseItems> {
    InternalWalletToDappInteractionPreAuthorizationResponseItems::new_with_subintent_and_signatures(
        subintent.into(),
        signatures.into_type(),
    )
    .into_result()
}

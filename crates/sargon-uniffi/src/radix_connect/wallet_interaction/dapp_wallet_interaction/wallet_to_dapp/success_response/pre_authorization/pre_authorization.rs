use crate::prelude::*;
use sargon::WalletToDappInteractionPreAuthorizationResponseItems as InternalWalletToDappInteractionPreAuthorizationResponseItems;
use sargon::WalletToDappInteractionSubintentResponseItem as InternalWalletToDappInteractionSubintentResponseItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionPreAuthorizationResponseItems {
    pub response: WalletToDappInteractionSubintentResponseItem,
}

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionSubintentResponseItem {
    /// A signed subintent
    pub signed_subintent: SignedSubintent,

    /// The timestamp at which the subintent expires
    pub expiration_timestamp: Timestamp,
}

#[uniffi::export]
pub fn new_wallet_to_dapp_interaction_pre_authorization_response_items(
    signed_subintent: SignedSubintent,
    expiration_timestamp: Timestamp,
) -> WalletToDappInteractionPreAuthorizationResponseItems {
    InternalWalletToDappInteractionPreAuthorizationResponseItems::new(
        signed_subintent.into(),
        expiration_timestamp,
    )
    .into()
}

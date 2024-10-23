use crate::prelude::*;
use sargon::WalletToDappInteractionPreAuthorizationResponseItems as InternalWalletToDappInteractionPreAuthorizationResponseItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionPreAuthorizationResponseItems {
    /// A hex encoded signed partial transaction.
    pub encoded_signed_partial_transaction: String,
}

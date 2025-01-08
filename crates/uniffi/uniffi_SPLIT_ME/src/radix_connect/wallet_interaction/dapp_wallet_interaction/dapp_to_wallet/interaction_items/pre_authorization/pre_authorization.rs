use crate::prelude::*;
use sargon::DappToWalletInteractionPreAuthorizationItems as InternalDappToWalletInteractionPreAuthorizationItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionPreAuthorizationItems {
    pub request: DappToWalletInteractionSubintentRequestItem,
}

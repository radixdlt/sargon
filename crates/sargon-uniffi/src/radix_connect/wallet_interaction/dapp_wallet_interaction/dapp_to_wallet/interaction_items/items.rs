use crate::prelude::*;
use sargon::DappToWalletInteractionItems as InternalDappToWalletInteractionItems;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Enum)]
pub enum DappToWalletInteractionItems {
    UnauthorizedRequest(DappToWalletInteractionUnauthorizedRequestItems),
    AuthorizedRequest(DappToWalletInteractionAuthorizedRequestItems),
    Transaction(DappToWalletInteractionTransactionItems),
}

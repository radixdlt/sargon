use crate::prelude::*;
use sargon::DappToWalletInteractionItems as InternalDappToWalletInteractionItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum DappToWalletInteractionItems {
    UnauthorizedRequest(DappToWalletInteractionUnauthorizedRequestItems),

    AuthorizedRequest(DappToWalletInteractionAuthorizedRequestItems),

    Transaction(DappToWalletInteractionTransactionItems),

    BatchOfTransactions(DappToWalletInteractionBatchOfTransactions),

    PreAuthorization(DappToWalletInteractionPreAuthorizationItems),
}

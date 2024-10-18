use crate::prelude::*;
use sargon::WalletToDappInteractionResponseItems as InternalWalletToDappInteractionResponseItems;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
#[allow(clippy::large_enum_variant)]
pub enum WalletToDappInteractionResponseItems {
    AuthorizedRequest(WalletToDappInteractionAuthorizedRequestResponseItems),
    UnauthorizedRequest(
        WalletToDappInteractionUnauthorizedRequestResponseItems,
    ),
    Transaction(WalletToDappInteractionTransactionResponseItems),
}

use crate::prelude::*;
use sargon::WalletToDappInteractionResponseItems as InternalWalletToDappInteractionResponseItems;

#[derive(Clone, PartialEq, uniffi::Enum)]
#[allow(clippy::large_enum_variant)]
pub enum WalletToDappInteractionResponseItems {
    AuthorizedRequest(WalletToDappInteractionAuthorizedRequestResponseItems),
    UnauthorizedRequest(
        WalletToDappInteractionUnauthorizedRequestResponseItems,
    ),
    Transaction(WalletToDappInteractionTransactionResponseItems),
}

impl From<InternalWalletToDappInteractionResponseItems>
    for WalletToDappInteractionResponseItems
{
    fn from(value: InternalWalletToDappInteractionResponseItems) -> Self {
        match value {
            InternalWalletToDappInteractionResponseItems::AuthorizedRequest(value) => WalletToDappInteractionResponseItems::AuthorizedRequest(value.into()),
            InternalWalletToDappInteractionResponseItems::UnauthorizedRequest(value) => WalletToDappInteractionResponseItems::UnauthorizedRequest(value.into()),
            InternalWalletToDappInteractionResponseItems::Transaction(value) => WalletToDappInteractionResponseItems::Transaction(value.into()),
        }
    }
}

impl Into<InternalWalletToDappInteractionResponseItems>
    for WalletToDappInteractionResponseItems
{
    fn into(self) -> InternalWalletToDappInteractionResponseItems {
        match self {
            WalletToDappInteractionResponseItems::AuthorizedRequest(value) => InternalWalletToDappInteractionResponseItems::AuthorizedRequest(value.into()),
            WalletToDappInteractionResponseItems::UnauthorizedRequest(value) => InternalWalletToDappInteractionResponseItems::UnauthorizedRequest(value.into()),
            WalletToDappInteractionResponseItems::Transaction(value) => InternalWalletToDappInteractionResponseItems::Transaction(value.into()),
        }
    }
}

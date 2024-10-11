use crate::prelude::*;
use sargon::DappToWalletInteractionItems as InternalDappToWalletInteractionItems;

#[derive(Clone, PartialEq, uniffi::Enum)]
pub enum DappToWalletInteractionItems {
    UnauthorizedRequest(DappToWalletInteractionUnauthorizedRequestItems),
    AuthorizedRequest(DappToWalletInteractionAuthorizedRequestItems),
    Transaction(DappToWalletInteractionTransactionItems),
}

impl From<InternalDappToWalletInteractionItems>
    for DappToWalletInteractionItems
{
    fn from(value: InternalDappToWalletInteractionItems) -> Self {
        match value {
            InternalDappToWalletInteractionItems::UnauthorizedRequest(
                value,
            ) => {
                DappToWalletInteractionItems::UnauthorizedRequest(value.into())
            }
            InternalDappToWalletInteractionItems::AuthorizedRequest(value) => {
                DappToWalletInteractionItems::AuthorizedRequest(value.into())
            }
            InternalDappToWalletInteractionItems::Transaction(value) => {
                DappToWalletInteractionItems::Transaction(value.into())
            }
        }
    }
}

impl Into<InternalDappToWalletInteractionItems>
    for DappToWalletInteractionItems
{
    fn into(self) -> InternalDappToWalletInteractionItems {
        match self {
            DappToWalletInteractionItems::UnauthorizedRequest(value) => {
                InternalDappToWalletInteractionItems::UnauthorizedRequest(
                    value.into(),
                )
            }
            DappToWalletInteractionItems::AuthorizedRequest(value) => {
                InternalDappToWalletInteractionItems::AuthorizedRequest(
                    value.into(),
                )
            }
            DappToWalletInteractionItems::Transaction(value) => {
                InternalDappToWalletInteractionItems::Transaction(value.into())
            }
        }
    }
}

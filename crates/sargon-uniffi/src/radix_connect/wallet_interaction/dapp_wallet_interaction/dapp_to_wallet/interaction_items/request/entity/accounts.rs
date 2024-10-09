use crate::prelude::*;
use sargon::DappToWalletInteractionAccountsRequestItem as InternalDappToWalletInteractionAccountsRequestItem;

#[derive(Debug, Clone, PartialEq,  uniffi::Record)]
pub struct DappToWalletInteractionAccountsRequestItem {
    pub number_of_accounts: RequestedQuantity,
    pub challenge: Option<DappToWalletInteractionAuthChallengeNonce>,
}

impl From<InternalDappToWalletInteractionAccountsRequestItem> for DappToWalletInteractionAccountsRequestItem {
    fn from(value: InternalDappToWalletInteractionAccountsRequestItem) -> Self {
        Self {
            number_of_accounts: value.number_of_accounts.into(),
            challenge: value.challenge.map(Into::into),
        }
    }
}

impl Into<InternalDappToWalletInteractionAccountsRequestItem> for DappToWalletInteractionAccountsRequestItem {
    fn into(self) -> InternalDappToWalletInteractionAccountsRequestItem {
        InternalDappToWalletInteractionAccountsRequestItem {
            number_of_accounts: self.number_of_accounts.into(),
            challenge: self.challenge.map(Into::into),
        }
    }
}
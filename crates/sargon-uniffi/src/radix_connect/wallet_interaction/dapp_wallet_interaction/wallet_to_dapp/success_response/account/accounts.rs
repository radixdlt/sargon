use crate::prelude::*;
use sargon::WalletToDappInteractionAccountsRequestResponseItem as InternalWalletToDappInteractionAccountsRequestResponseItem;

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct WalletToDappInteractionAccountsRequestResponseItem {
    pub accounts: Vec<WalletInteractionWalletAccount>,
    pub challenge: Option<DappToWalletInteractionAuthChallengeNonce>,
    pub proofs: Option<Vec<WalletToDappInteractionAccountProof>>,
}

impl From<InternalWalletToDappInteractionAccountsRequestResponseItem> for WalletToDappInteractionAccountsRequestResponseItem {
    fn from(value: InternalWalletToDappInteractionAccountsRequestResponseItem) -> Self {
        Self {
            accounts: value.accounts.into_vec(),
            challenge: value.challenge.map(From::from),
            proofs: value.proofs.map(|proofs| proofs.into_vec()),
        }
    }
}

impl Into<InternalWalletToDappInteractionAccountsRequestResponseItem> for WalletToDappInteractionAccountsRequestResponseItem {
    fn into(self) -> InternalWalletToDappInteractionAccountsRequestResponseItem {
        InternalWalletToDappInteractionAccountsRequestResponseItem {
            accounts: self.accounts.into_internal_vec(),
            challenge: self.challenge.map(Into::into),
            proofs: self.proofs.map(|proofs| proofs.into_internal_vec()),
        }
    }
}
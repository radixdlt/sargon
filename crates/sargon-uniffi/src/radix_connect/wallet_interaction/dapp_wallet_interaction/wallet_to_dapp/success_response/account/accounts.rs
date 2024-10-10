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
            accounts: value.accounts.into_iter().map(Into::into).collect(),
            challenge: value.challenge,
            proofs: value.proofs.map(|proofs| proofs.into_iter().map(Into::into).collect()),
        }
    }
}

impl Into<InternalWalletToDappInteractionAccountsRequestResponseItem> for WalletToDappInteractionAccountsRequestResponseItem {
    fn into(self) -> InternalWalletToDappInteractionAccountsRequestResponseItem {
        InternalWalletToDappInteractionAccountsRequestResponseItem {
            accounts: self.accounts.into_iter().map(Into::into).collect(),
            challenge: self.challenge,
            proofs: self.proofs.map(|proofs| proofs.into_iter().map(Into::into).collect()),
        }
    }
}
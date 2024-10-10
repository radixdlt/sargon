use crate::prelude::*;
use sargon::DappToWalletInteractionAuthLoginWithChallengeRequestItem as InternalDappToWalletInteractionAuthLoginWithChallengeRequestItem;

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
}

impl From<InternalDappToWalletInteractionAuthLoginWithChallengeRequestItem> for DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    fn from(value: InternalDappToWalletInteractionAuthLoginWithChallengeRequestItem) -> Self {
        Self {
            challenge: value.challenge.into(),
        }
    }
}

impl Into<InternalDappToWalletInteractionAuthLoginWithChallengeRequestItem> for DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    fn into(self) -> InternalDappToWalletInteractionAuthLoginWithChallengeRequestItem {
        InternalDappToWalletInteractionAuthLoginWithChallengeRequestItem {
            challenge: self.challenge.into(),
        }
    }
}

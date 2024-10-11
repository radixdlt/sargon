use crate::prelude::*;
use sargon::WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem as InternalWalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem;

#[derive(Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}

impl From<InternalWalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem> for WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem {
    fn from(value: InternalWalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem) -> Self {
        Self {
            persona: value.persona.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem> for WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem {
    fn into(self) -> InternalWalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem {
        InternalWalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem {
            persona: self.persona.into(),
        }
    }
}

use crate::prelude::*;
use sargon::WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem as InternalWalletToDappInteractionAuthLoginWithChallengeRequestResponseItem;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
    pub proof: WalletToDappInteractionAuthProof,
}

impl From<InternalWalletToDappInteractionAuthLoginWithChallengeRequestResponseItem> for WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    fn from(value: InternalWalletToDappInteractionAuthLoginWithChallengeRequestResponseItem) -> Self {
        Self {
            persona: value.persona.into(),
            challenge: value.challenge.into(),
            proof: value.proof.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionAuthLoginWithChallengeRequestResponseItem> for WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    fn into(self) -> InternalWalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
        InternalWalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
            persona: self.persona.into(),
            challenge: self.challenge.into(),
            proof: self.proof.into(),
        }
    }
}
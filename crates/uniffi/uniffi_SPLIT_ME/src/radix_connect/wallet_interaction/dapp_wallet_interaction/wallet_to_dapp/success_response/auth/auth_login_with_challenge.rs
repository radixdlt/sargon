use crate::prelude::*;
use sargon::WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem as InternalWalletToDappInteractionAuthLoginWithChallengeRequestResponseItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
    pub proof: WalletToDappInteractionAuthProof,
}

use crate::prelude::*;
use sargon::WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem as InternalWalletToDappInteractionAuthLoginWithChallengeRequestResponseItem;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
    pub proof: WalletToDappInteractionAuthProof,
}

use crate::prelude::*;
use sargon::WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem as InternalWalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}
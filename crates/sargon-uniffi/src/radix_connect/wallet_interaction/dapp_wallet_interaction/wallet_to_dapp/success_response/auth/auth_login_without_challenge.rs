use crate::prelude::*;
use sargon::WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem as InternalWalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
}

use crate::prelude::*;
use sargon::WalletToDappInteractionAuthRequestResponseItem as InternalWalletToDappInteractionAuthRequestResponseItem;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum WalletToDappInteractionAuthRequestResponseItem {
    UsePersona(WalletToDappInteractionAuthUsePersonaRequestResponseItem),
    LoginWithoutChallenge(
        WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem,
    ),
    LoginWithChallenge(
        WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem,
    ),
}

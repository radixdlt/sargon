use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
pub enum DappWalletInteractionAuthRequestResponseItem {
    #[serde(rename = "usePersona")]
    UsePersona(DappWalletInteractionAuthUsePersonaRequestResponseItem),
    #[serde(rename = "loginWithoutChallenge")]
    LoginWithoutChallenge(
        DappWalletInteractionAuthLoginWithoutChallengeRequestResponseItem,
    ),
    #[serde(rename = "loginWithChallenge")]
    LoginWithChallenge(
        DappWalletInteractionAuthLoginWithChallengeRequestResponseItem,
    ),
}

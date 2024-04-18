use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
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

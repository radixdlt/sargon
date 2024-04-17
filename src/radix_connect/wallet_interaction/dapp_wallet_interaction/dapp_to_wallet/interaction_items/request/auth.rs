use super::auth_use_persona::DappToWalletInteractionAuthUsePersonaRequestItem;
use super::login_with_challenge::DappToWalletInteractionAuthLoginWithChallengeRequestItem;
use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
pub enum DappToWalletInteractionAuthRequestItem {
    #[serde(rename = "loginWithChallenge")]
    LoginWithChallenge(
        DappToWalletInteractionAuthLoginWithChallengeRequestItem,
    ),
    #[serde(rename = "loginWithoutChallenge")]
    LoginWithoutChallenge,
    #[serde(rename = "usePersona")]
    UsePersona(DappToWalletInteractionAuthUsePersonaRequestItem),
}

impl HasSampleValues for DappToWalletInteractionAuthRequestItem {
    fn sample() -> Self {
        Self::LoginWithChallenge(
            DappToWalletInteractionAuthLoginWithChallengeRequestItem::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::UsePersona(
            DappToWalletInteractionAuthUsePersonaRequestItem::sample(),
        )
    }
}

use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
pub enum WalletToDappInteractionAuthRequestResponseItem {
    #[serde(rename = "usePersona")]
    UsePersona(WalletToDappInteractionAuthUsePersonaRequestResponseItem),
    #[serde(rename = "loginWithoutChallenge")]
    LoginWithoutChallenge(
        WalletToDappInteractionAuthLoginWithoutChallengeRequestResponseItem,
    ),
    #[serde(rename = "loginWithChallenge")]
    LoginWithChallenge(
        WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem,
    ),
}

impl HasSampleValues for WalletToDappInteractionAuthRequestResponseItem {
    fn sample() -> Self {
        WalletToDappInteractionAuthRequestResponseItem::UsePersona(
            WalletToDappInteractionAuthUsePersonaRequestResponseItem::sample(),
        )
    }

    fn sample_other() -> Self {
        WalletToDappInteractionAuthRequestResponseItem::LoginWithChallenge(
            WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem::sample_other(),
        )
    }
}

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, uniffi::Enum)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionAuthRequestResponseItem;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}

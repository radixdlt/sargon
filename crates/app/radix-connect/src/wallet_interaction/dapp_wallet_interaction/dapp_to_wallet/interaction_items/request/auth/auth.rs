use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionAuthRequestItem;

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

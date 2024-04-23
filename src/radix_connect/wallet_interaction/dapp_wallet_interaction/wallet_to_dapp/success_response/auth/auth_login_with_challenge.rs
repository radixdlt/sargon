use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: Exactly32Bytes,
    pub proof: WalletToDappInteractionAuthProof,
}

impl WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    pub fn new(
        persona: DappWalletInteractionPersona,
        challenge: impl Into<Exactly32Bytes>,
        proof: WalletToDappInteractionAuthProof,
    ) -> Self {
        Self {
            persona,
            challenge: challenge.into(),
            proof,
        }
    }
}

impl HasSampleValues
    for WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem
{
    fn sample() -> Self {
        Self::new(
            DappWalletInteractionPersona::sample(),
            Exactly32Bytes::sample(),
            WalletToDappInteractionAuthProof::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            DappWalletInteractionPersona::sample_other(),
            Exactly32Bytes::sample_other(),
            WalletToDappInteractionAuthProof::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem;

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

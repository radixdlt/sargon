use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    pub persona: DappWalletInteractionPersona,
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
    pub proof: WalletToDappInteractionAuthProof,
}

impl WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem {
    pub fn new(
        persona: DappWalletInteractionPersona,
        challenge: impl Into<DappToWalletInteractionAuthChallengeNonce>,
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
            DappToWalletInteractionAuthChallengeNonce::sample(),
            WalletToDappInteractionAuthProof::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            DappWalletInteractionPersona::sample_other(),
            DappToWalletInteractionAuthChallengeNonce::sample_other(),
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

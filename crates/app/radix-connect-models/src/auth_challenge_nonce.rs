use crate::prelude::*;

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd, Hash,
)]
pub struct DappToWalletInteractionAuthChallengeNonce(pub Exactly32Bytes);

impl HasSampleValues for DappToWalletInteractionAuthChallengeNonce {
    fn sample() -> Self {
        DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::sample())
    }

    fn sample_other() -> Self {
        DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionAuthChallengeNonce;

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

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    pub challenge: Exactly32Bytes,
}

impl DappToWalletInteractionAuthLoginWithChallengeRequestItem {
    pub fn new(challenge: impl Into<Exactly32Bytes>) -> Self {
        Self {
            challenge: challenge.into(),
        }
    }
}

impl HasSampleValues
    for DappToWalletInteractionAuthLoginWithChallengeRequestItem
{
    fn sample() -> Self {
        Self::new(Exactly32Bytes::sample())
    }

    fn sample_other() -> Self {
        Self::new(Exactly32Bytes::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionAuthLoginWithChallengeRequestItem;

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

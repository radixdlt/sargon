use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct DappRequest {
    pub interaction_id: WalletInteractionId,
    pub session_id: SessionID,
}

impl HasSampleValues for DappRequest {
    fn sample() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample(),
            session_id: SessionID::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample_other(),
            session_id: SessionID::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappRequest;

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

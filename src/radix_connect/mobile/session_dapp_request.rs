use crate::prelude::*;

#[derive(Debug, PartialEq, uniffi::Record)]
pub struct RadixConnectMobileSessionRequest {
    pub session_id: SessionID,
    pub interaction: DappToWalletInteractionUnvalidated,
}

impl RadixConnectMobileSessionRequest {
    pub fn new(
        session_id: impl Into<SessionID>,
        interaction: DappToWalletInteractionUnvalidated,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            interaction,
        }
    }
}

impl HasSampleValues for RadixConnectMobileSessionRequest {
    fn sample() -> Self {
        Self::new(
            SessionID::sample(),
            DappToWalletInteractionUnvalidated::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SessionID::sample_other(),
            DappToWalletInteractionUnvalidated::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectMobileSessionRequest;

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
use crate::prelude::*;

#[derive(Debug, PartialEq, uniffi::Record)]
pub struct RadixConnectMobileWalletResponse {
    pub session_id: SessionID,
    pub response: WalletToDappInteractionResponse,
}

impl RadixConnectMobileWalletResponse {
    pub fn new(
        session_id: impl Into<SessionID>,
        response: WalletToDappInteractionResponse,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            response,
        }
    }
}

impl HasSampleValues for RadixConnectMobileWalletResponse {
    fn sample() -> Self {
        Self::new(
            SessionID::sample(),
            WalletToDappInteractionResponse::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SessionID::sample_other(),
            WalletToDappInteractionResponse::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectMobileWalletResponse;

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

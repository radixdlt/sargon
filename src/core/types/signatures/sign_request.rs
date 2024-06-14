use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Enum)]
pub enum SignRequest {
    Transaction(SignTransactionRequest),
    AuthChallenge(SignAuthChallengeRequest)
}

impl HasSampleValues for SignRequest {
    fn sample() -> Self {
        Self::Transaction(SignTransactionRequest::sample())
    }

    fn sample_other() -> Self {
        Self::AuthChallenge(SignAuthChallengeRequest::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignRequest;

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
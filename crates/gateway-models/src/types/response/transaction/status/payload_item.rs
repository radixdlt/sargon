use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct TransactionStatusResponsePayloadItem {
    pub payload_status: Option<TransactionStatusResponsePayloadStatus>,
}

impl HasSampleValues for TransactionStatusResponsePayloadItem {
    fn sample() -> Self {
        Self::sample_pending()
    }

    fn sample_other() -> Self {
        Self::sample_committed_success()
    }
}

impl TransactionStatusResponsePayloadItem {
    pub fn sample_unknown() -> Self {
        Self {
            payload_status: Some(
                TransactionStatusResponsePayloadStatus::Unknown,
            ),
        }
    }

    pub fn sample_commit_pending_outcome_unknown() -> Self {
        Self {
            payload_status: Some(
                TransactionStatusResponsePayloadStatus::CommitPendingOutcomeUnknown,
            ),
        }
    }

    pub fn sample_pending() -> Self {
        Self {
            payload_status: Some(
                TransactionStatusResponsePayloadStatus::Pending,
            ),
        }
    }

    pub fn sample_committed_success() -> Self {
        Self {
            payload_status: Some(
                TransactionStatusResponsePayloadStatus::CommittedSuccess,
            ),
        }
    }

    pub fn sample_committed_failure() -> Self {
        Self {
            payload_status: Some(
                TransactionStatusResponsePayloadStatus::CommittedFailure,
            ),
        }
    }

    pub fn sample_committed_permanently_rejected() -> Self {
        Self {
            payload_status: Some(
                TransactionStatusResponsePayloadStatus::PermanentlyRejected,
            ),
        }
    }

    pub fn sample_temporarily_rejected() -> Self {
        Self {
            payload_status: Some(
                TransactionStatusResponsePayloadStatus::TemporarilyRejected,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionStatusResponsePayloadItem;

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

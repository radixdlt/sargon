use crate::prelude::*;

impl HasSampleValues for TransactionStatusResponsePayloadItem {
    fn sample() -> Self {
        Self::sample_pending()
    }

    fn sample_other() -> Self {
        Self::sample_committed_success()
    }
}

impl TransactionStatusResponsePayloadItem {
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

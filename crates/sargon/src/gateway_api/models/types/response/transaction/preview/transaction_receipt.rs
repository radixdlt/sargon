use crate::prelude::*;

/// This is part of the response to a transaction preview request, and contains the status of the transaction.
/// Error message is only present if status is `Failed` or `Rejected`.
#[derive(
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{status}")]
pub struct TransactionReceipt {
    pub status: TransactionReceiptStatus,
    pub error_message: Option<String>,
}

impl HasSampleValues for TransactionReceipt {
    fn sample() -> Self {
        Self {
            status: TransactionReceiptStatus::Succeeded,
            error_message: None,
        }
    }

    fn sample_other() -> Self {
        Self {
            status: TransactionReceiptStatus::Failed,
            error_message: Some("An error occurred".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionReceipt;

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

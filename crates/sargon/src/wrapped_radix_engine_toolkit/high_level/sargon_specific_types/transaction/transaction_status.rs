use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus {
    /// The transaction has been successfully processed and is now final.
    Success,

    /// The transaction has been permanently rejected with the given `reason`.
    PermanentlyRejected { reason: TransactionStatusReason },

    /// The transaction has been temporarily rejected and may be processed in the future.
    TemporarilyRejected { current_epoch: Epoch },

    /// The transaction has failed with the given `reason`.
    Failed { reason: TransactionStatusReason },
}

impl HasSampleValues for TransactionStatus {
    fn sample() -> Self {
        Self::Success
    }

    fn sample_other() -> Self {
        Self::PermanentlyRejected {
            reason: TransactionStatusReason::sample(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionReceiptStatus;

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

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
pub enum TransactionStatusReason {
    /// The transaction was rejected for an unknown reason.
    Unknown,

    /// The transaction was rejected because there was an application error in the worktop.
    WorktopError,
}

impl HasSampleValues for TransactionStatusReason {
    fn sample() -> Self {
        Self::Unknown
    }

    fn sample_other() -> Self {
        Self::WorktopError
    }
}

impl TransactionStatusReason {
    pub fn from_raw_error(raw_error: impl Into<Option<String>>) -> Self {
        match raw_error.into() {
            Some(raw_error) => {
                if raw_error.contains("AssertionFailed") {
                    Self::WorktopError
                } else {
                    Self::Unknown
                }
            }
            None => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionStatusReason;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_error() {
        let mut sut = SUT::from_raw_error(None);
        assert_eq!(sut, SUT::Unknown);

        sut = SUT::from_raw_error("whatever".to_string());
        assert_eq!(sut, SUT::Unknown);

        sut = SUT::from_raw_error("AssertionFailed".to_string());
        assert_eq!(sut, SUT::WorktopError);
    }
}

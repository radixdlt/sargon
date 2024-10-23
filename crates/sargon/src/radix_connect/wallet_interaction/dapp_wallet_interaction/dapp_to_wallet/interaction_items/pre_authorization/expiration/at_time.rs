use crate::prelude::*;

/// The subintent expires at a specific fixed timestamp
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSubintentExpireAtTime {
    /// The unix timestamp in seconds when the subintent expires.
    pub unix_timestamp_seconds: Timestamp,
}

impl From<Timestamp> for DappToWalletInteractionSubintentExpireAtTime {
    fn from(unix_timestamp_seconds: Timestamp) -> Self {
        Self {
            unix_timestamp_seconds,
        }
    }
}

impl HasSampleValues for DappToWalletInteractionSubintentExpireAtTime {
    fn sample() -> Self {
        Self {
            unix_timestamp_seconds: Timestamp::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            unix_timestamp_seconds: Timestamp::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionSubintentExpireAtTime;

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
    fn from() {
        assert_eq!(SUT::from(Timestamp::sample()), SUT::sample());
    }
}

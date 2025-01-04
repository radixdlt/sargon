use crate::prelude::*;

/// Suggests that the subintent's expiry timestamp is set to `current_time + expire_after_seconds`
/// at the last moment, right before the intent is fixed for signing.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSubintentExpireAfterDelay {
    /// The time (in seconds) after the subintent is signed that it will expire.
    pub expire_after_seconds: u64,
}

impl From<u64> for DappToWalletInteractionSubintentExpireAfterDelay {
    fn from(expire_after_seconds: u64) -> Self {
        Self {
            expire_after_seconds,
        }
    }
}

impl HasSampleValues for DappToWalletInteractionSubintentExpireAfterDelay {
    fn sample() -> Self {
        Self {
            expire_after_seconds: 10,
        }
    }

    fn sample_other() -> Self {
        Self {
            expire_after_seconds: 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionSubintentExpireAfterDelay;

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
        assert_eq!(SUT::from(10), SUT::sample());
    }
}

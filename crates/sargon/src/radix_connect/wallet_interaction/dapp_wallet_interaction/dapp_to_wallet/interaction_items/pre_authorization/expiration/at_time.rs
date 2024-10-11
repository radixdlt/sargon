use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSubintentExpirationAtTime {
    /// The time at which the expiration should occur.
    pub value: Timestamp,
}

impl DappToWalletInteractionSubintentExpirationAtTime {
    pub fn new(value: impl Into<Timestamp>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<Timestamp> for DappToWalletInteractionSubintentExpirationAtTime {
    fn from(value: Timestamp) -> Self {
        Self::new(value)
    }
}

impl HasSampleValues for DappToWalletInteractionSubintentExpirationAtTime {
    fn sample() -> Self {
        Self::new(Timestamp::sample())
    }

    fn sample_other() -> Self {
        Self::new(Timestamp::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionSubintentExpirationAtTime;

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
        assert_eq!(SUT::from(Timestamp::sample_other()), SUT::sample_other(),);
    }

    #[test]
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
        {
            "value": "2023-09-11T16:05:56.000Z"
        }
        "#,
        );
    }
}

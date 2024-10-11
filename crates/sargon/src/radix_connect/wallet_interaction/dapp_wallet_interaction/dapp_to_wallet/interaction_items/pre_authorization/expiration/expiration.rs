use crate::prelude::*;

/// An enum
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
#[serde(rename_all = "camelCase")]
pub enum DappToWalletInteractionSubintentExpiration {
    #[serde(rename = "expireAtTime")]
    AtTime(DappToWalletInteractionSubintentExpirationAtTime),

    #[serde(rename = "expireAfterSignature")]
    AfterSignature(DappToWalletInteractionSubintentExpirationAfterSignature),
}

impl From<DappToWalletInteractionSubintentExpirationAtTime>
    for DappToWalletInteractionSubintentExpiration
{
    fn from(value: DappToWalletInteractionSubintentExpirationAtTime) -> Self {
        Self::AtTime(value)
    }
}

impl From<DappToWalletInteractionSubintentExpirationAfterSignature>
    for DappToWalletInteractionSubintentExpiration
{
    fn from(
        value: DappToWalletInteractionSubintentExpirationAfterSignature,
    ) -> Self {
        Self::AfterSignature(value)
    }
}

impl HasSampleValues for DappToWalletInteractionSubintentExpiration {
    fn sample() -> Self {
        Self::AtTime(DappToWalletInteractionSubintentExpirationAtTime::sample())
    }

    fn sample_other() -> Self {
        Self::AfterSignature(
            DappToWalletInteractionSubintentExpirationAfterSignature::sample(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionSubintentExpiration;

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
        let sample: SUT = DappToWalletInteractionSubintentExpirationAtTime::sample().into();
        assert_eq!(sample, SUT::sample());

        let sample_other: SUT = DappToWalletInteractionSubintentExpirationAfterSignature::sample().into();
        assert_eq!(sample_other, SUT::sample_other());
    }

    #[test]
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(&SUT::sample(), r#"
        {
            "discriminator": "expireAtTime",
            "value": "2023-09-11T16:05:56.000Z"
        }
        "#,);

        assert_eq_after_json_roundtrip(&SUT::sample_other(), r#"
        {
            "discriminator": "expireAfterSignature",
            "value": 10
        }
        "#,);
    }
}

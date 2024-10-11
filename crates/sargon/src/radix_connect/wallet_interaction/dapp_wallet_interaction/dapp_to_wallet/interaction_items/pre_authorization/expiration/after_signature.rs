use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSubintentExpirationAfterSignature {
    /// The number in seconds after the signature that the expiration should occur.
    pub value: u64,
}

impl DappToWalletInteractionSubintentExpirationAfterSignature {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

impl From<u64> for DappToWalletInteractionSubintentExpirationAfterSignature {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl HasSampleValues
    for DappToWalletInteractionSubintentExpirationAfterSignature
{
    fn sample() -> Self {
        Self::new(10)
    }

    fn sample_other() -> Self {
        Self::new(20)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionSubintentExpirationAfterSignature;

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
        assert_eq!(SUT::from(20), SUT::sample_other(),);
    }

    #[test]
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
        {
            "value": 10
        }
        "#,
        );
    }
}

use crate::prelude::*;

/// An enum that represents the different ways a subintent can expire.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator", content = "value", rename_all = "camelCase")]
pub enum DappToWalletInteractionSubintentExpiration {
    /// The subintent expires at a specific time.
    ///
    /// For example, a dApp sends a subintent for `User A` to approve sending 100 XRD before 5:00 PM,
    /// and a subintent for `User B` to approve sending 2 USDT with same expiration.
    ///
    /// If both users sign their subintents before 5:00 PM, the transaction to exchange
    /// 100 XRD over 2 USDT will succeed. Otherwise, it would fail.
    #[serde(rename = "expireAtTime")]
    AtTime(Timestamp),

    /// The subintent expires X seconds after its signature.
    ///
    /// For example, a dApp sends a subintent for `User A` to approve sending 100 XRD with 1 hour expiration,
    /// and a subintent for `User B` to approve sending 2 USDT with same expiration.
    ///
    /// If both users sign their subintents within one hour from each other, the transaction to exchange
    /// 100 XRD over 2 USDT will succeed. Otherwise, it would fail.
    #[serde(rename = "expireAfterSignature")]
    AllSubintentsSubmittedWithin(u64),
}

impl HasSampleValues for DappToWalletInteractionSubintentExpiration {
    fn sample() -> Self {
        Self::AtTime(Timestamp::sample())
    }

    fn sample_other() -> Self {
        Self::AllSubintentsSubmittedWithin(10)
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
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
        {
            "discriminator": "expireAtTime",
            "value": "2023-09-11T16:05:56.000Z"
        }
        "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r#"
        {
            "discriminator": "expireAfterSignature",
            "value": 10
        }
        "#,
        );
    }
}

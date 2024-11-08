use crate::prelude::*;
use std::time::Duration;

/// An enum that represents the different ways a subintent can expire.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "discriminator", rename_all = "camelCase")]
pub enum DappToWalletInteractionSubintentExpiration {
    /// The subintent expires at a specific fixed timestamp.
    ///
    /// For example, a dApp sends a subintent for `User A` to approve sending 100 XRD before 5:00 PM,
    /// and a subintent for `User B` to approve sending 2 USDT with same expiration.
    ///
    /// If both users sign their subintents before 5:00 PM, the transaction to exchange
    /// 100 XRD over 2 USDT will succeed. Otherwise, it would fail.
    #[serde(rename = "expireAtTime")]
    AtTime(DappToWalletInteractionSubintentExpireAtTime),

    /// The subintent expires X seconds after its signature.
    ///
    /// For example, a dApp sends a subintent for `User A` to approve sending 100 XRD with 1 hour expiration,
    /// and a subintent for `User B` to approve sending 2 USDT with same expiration.
    ///
    /// If both users sign their subintents within one hour from each other, the transaction to exchange
    /// 100 XRD over 2 USDT will succeed. Otherwise, it would fail.
    #[serde(rename = "expireAfterDelay")]
    AfterDelay(DappToWalletInteractionSubintentExpireAfterDelay),
}

impl HasSampleValues for DappToWalletInteractionSubintentExpiration {
    fn sample() -> Self {
        Self::AtTime(DappToWalletInteractionSubintentExpireAtTime::sample())
    }

    fn sample_other() -> Self {
        Self::AfterDelay(
            DappToWalletInteractionSubintentExpireAfterDelay::sample(),
        )
    }
}

impl DappToWalletInteractionSubintentExpiration {
    pub fn get_status(
        &self,
    ) -> DappToWalletInteractionSubintentExpirationStatus {
        match self {
            Self::AtTime(expiration) => {
                let now = seconds_since_unix_epoch();
                let in_thirty_seconds = now + 30;
                if expiration.unix_timestamp_seconds < now {
                    DappToWalletInteractionSubintentExpirationStatus::Expired
                } else if expiration.unix_timestamp_seconds < in_thirty_seconds
                {
                    DappToWalletInteractionSubintentExpirationStatus::ExpirationTooClose
                } else {
                    DappToWalletInteractionSubintentExpirationStatus::Valid
                }
            }
            Self::AfterDelay(_) => {
                DappToWalletInteractionSubintentExpirationStatus::Valid
            }
        }
    }
}

// Returns the amounts of seconds since the Unix epoch.
pub fn seconds_since_unix_epoch() -> u64 {
    Timestamp::now_utc()
        .duration_since(Timestamp::UNIX_EPOCH)
        .as_seconds_f64() as u64
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
            "unixTimestampSeconds": 1730999831257
        }
        "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r#"
        {
            "discriminator": "expireAfterDelay",
            "expireAfterSeconds": 10
        }
        "#,
        );
    }

    #[test]
    fn status() {
        let now = Timestamp::now_utc();

        // AtTime which has already expired
        let now = seconds_since_unix_epoch();
        let past = now - 30;
        let expiration = SUT::AtTime(past.into());
        assert_eq!(
            expiration.get_status(),
            DappToWalletInteractionSubintentExpirationStatus::Expired
        );

        // AtTime which is less than 30 seconds from expiration
        let in_ten_seconds = now + 10;
        let expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            DappToWalletInteractionSubintentExpireAtTime {
                unix_timestamp_seconds: in_ten_seconds,
            },
        );
        assert_eq!(
            expiration.get_status(),
            DappToWalletInteractionSubintentExpirationStatus::ExpirationTooClose
        );

        // AtTime which is more than 30 seconds from expiration
        let in_forty_seconds = now + 40;
        let expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            DappToWalletInteractionSubintentExpireAtTime {
                unix_timestamp_seconds: in_forty_seconds,
            },
        );
        assert_eq!(
            expiration.get_status(),
            DappToWalletInteractionSubintentExpirationStatus::Valid
        );

        // AfterDelay is always Valid, either in 10 minutes
        let expiration = SUT::AfterDelay(600.into());
        assert_eq!(
            expiration.get_status(),
            DappToWalletInteractionSubintentExpirationStatus::Valid
        );

        // .. or in 15 seconds
        let expiration = SUT::AfterDelay(15.into());
        assert_eq!(
            expiration.get_status(),
            DappToWalletInteractionSubintentExpirationStatus::Valid
        );
    }
}

pub use crate::prelude::*;

/// Represents a Unix timestamp, capturing the seconds since the unix epoch.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
pub struct Instant {
    pub seconds_since_unix_epoch: i64,
}

impl From<i64> for Instant {
    fn from(value: i64) -> Self {
        Self {
            seconds_since_unix_epoch: value,
        }
    }
}

impl From<Instant> for i64 {
    fn from(value: Instant) -> Self {
        value.seconds_since_unix_epoch
    }
}

impl From<Instant> for ScryptoInstant {
    fn from(value: Instant) -> Self {
        Self::new(value.seconds_since_unix_epoch)
    }
}

impl From<ScryptoInstant> for Instant {
    fn from(value: ScryptoInstant) -> Self {
        Self {
            seconds_since_unix_epoch: value.seconds_since_unix_epoch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn into_from_scrypto() {
        let test = |i: i64| {
            assert_eq!(
                Instant::from(ScryptoInstant::new(i)).seconds_since_unix_epoch,
                i
            )
        };
        test(-10);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn from_i64() {
        let test = |i: i64| assert_eq!(i64::from(Instant::from(i)), i);
        test(-10);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn to_i64() {
        let test = |i: i64| {
            assert_eq!(
                Instant::from(i64::from(Instant::from(i)))
                    .seconds_since_unix_epoch,
                i
            )
        };
        test(-10);
        test(0);
        test(1);
        test(2);
        test(1337);
    }
}

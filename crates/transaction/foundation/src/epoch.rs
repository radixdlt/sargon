pub use crate::prelude::*;

/// A type-safe consensus epoch number.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Ord,
    PartialOrd,
    derive_more::Display,
    derive_more::Debug,
)]
pub struct Epoch(pub EpochInner);

impl Epoch {
    /// Circa 1 hour, since one epoch is circa 6 minutes.
    pub const NUMBER_OF_EPOCHS_PER_HOUR: EpochInner = 10;
    pub const NUMBER_OF_EPOCHS_PER_DAY: EpochInner =
        Self::NUMBER_OF_EPOCHS_PER_HOUR * 24;
    pub const NUMBER_OF_EPOCHS_PER_WEEK: EpochInner =
        Self::NUMBER_OF_EPOCHS_PER_DAY * 7;

    pub const DEFAULT_EPOCH_WINDOW_SIZE: EpochInner =
        Self::NUMBER_OF_EPOCHS_PER_HOUR;

    pub const NUMBER_OF_EPOCHS_PER_MONTH: EpochInner =
        Self::NUMBER_OF_EPOCHS_PER_DAY * 30;

    pub const MAX_EPOCH_WINDOW: EpochInner = Self::NUMBER_OF_EPOCHS_PER_MONTH;
}

pub type EpochInner = u64;
impl Epoch {
    pub fn new(value: impl Into<EpochInner>) -> Self {
        Self(value.into())
    }

    pub fn window_end_from_start(start: Self) -> Self {
        Self::new(start.0 + Self::DEFAULT_EPOCH_WINDOW_SIZE)
    }

    pub fn adding(&self, amount: EpochInner) -> Self {
        Self::new(self.0 + amount)
    }

    pub fn max_window_from_start(start: Self) -> Self {
        Self::new(start.0 + Self::MAX_EPOCH_WINDOW)
    }
}

impl From<EpochInner> for Epoch {
    fn from(value: EpochInner) -> Self {
        Self::new(value)
    }
}

impl From<Epoch> for EpochInner {
    fn from(value: Epoch) -> Self {
        value.0
    }
}

impl From<Epoch> for ScryptoEpoch {
    fn from(value: Epoch) -> Self {
        Self::of(value.0)
    }
}

impl From<ScryptoEpoch> for Epoch {
    fn from(value: ScryptoEpoch) -> Self {
        Self::new(value.number())
    }
}

impl HasSampleValues for Epoch {
    fn sample() -> Self {
        Self(0)
    }

    fn sample_other() -> Self {
        Self(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Epoch;

    #[test]
    fn into_from_scrypto() {
        let test = |u: u64| assert_eq!(SUT::from(ScryptoEpoch::of(u)).0, u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn from_u64() {
        let test = |u: u64| assert_eq!(u64::from(SUT::from(u)), u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn to_u64() {
        let test = |u: u64| assert_eq!(SUT::from(u64::from(SUT::from(u))).0, u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn adding() {
        let sut = SUT::new(10u64);
        assert_eq!(sut.adding(5), SUT::new(15u64));
    }

    #[test]
    fn max_is_one_month() {
        let start = SUT::new(10u64);
        assert_eq!(
            SUT::max_window_from_start(start).0,
            10 + SUT::NUMBER_OF_EPOCHS_PER_MONTH
        );
    }
}

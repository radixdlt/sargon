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
pub struct Epoch(pub u64);

impl Epoch {
    /// Circa 1 hour, since one epoch is circa 6 minutes.
    pub const NUMBER_OF_EPOCHS_PER_HOUR: u64 = 10;
    pub const NUMBER_OF_EPOCHS_PER_DAY: u64 = Self::NUMBER_OF_EPOCHS_PER_HOUR * 24;
    pub const NUMBER_OF_EPOCHS_PER_WEEK: u64 = Self::NUMBER_OF_EPOCHS_PER_DAY * 7;

     pub const DEFAULT_EPOCH_WINDOW_SIZE: u64 = Self::NUMBER_OF_EPOCHS_PER_HOUR;
}

impl Epoch {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn window_end_from_start(start: Self) -> Self {
        Self::new(start.0 + Self::DEFAULT_EPOCH_WINDOW_SIZE)
    }

    pub fn adding(&self, amount: u64) -> Self {
        Self::new(self.0 + amount)
    }

    pub fn one_week_from(start: Self) -> Self {
        Self::new(start.0 + Self::NUMBER_OF_EPOCHS_PER_WEEK)
    }
}

impl From<u64> for Epoch {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Epoch> for u64 {
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

    #[test]
    fn into_from_scrypto() {
        let test = |u: u64| assert_eq!(Epoch::from(ScryptoEpoch::of(u)).0, u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn from_u64() {
        let test = |u: u64| assert_eq!(u64::from(Epoch::from(u)), u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn to_u64() {
        let test =
            |u: u64| assert_eq!(Epoch::from(u64::from(Epoch::from(u))).0, u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn adding() {
        let sut = Epoch(10);
        assert_eq!(sut.adding(5), Epoch(15));
    }
}

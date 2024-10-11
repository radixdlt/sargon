pub use crate::prelude::*;

/// A type-safe consensus epoch number.
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
)]
pub struct Epoch(pub u64);

impl From<u64> for Epoch {
    fn from(value: u64) -> Self {
        Self(value)
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
        Self(value.number())
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
    use crate::prelude::*;

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
}

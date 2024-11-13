use crate::prelude::*;

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef,
)]
pub struct U30(pub u32);

impl U30 {
    pub const MAX: u32 = U30_MAX;

    pub(crate) const fn new(value: u16) -> Self {
        Self(value as u32)
    }

    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
}

impl AddViaDeref for U30 {}
impl AddSelfViaDeref for U30 {}

impl From<u16> for U30 {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<U30> for U31 {
    fn from(value: U30) -> Self {
        Self::try_from(value.0).unwrap()
    }
}
impl TryFrom<U31> for U30 {
    type Error = CommonError;
    fn try_from(value: U31) -> Result<Self, Self::Error> {
        let large: u32 = value.into();
        Self::try_from(large)
    }
}

impl TryFrom<u32> for U30 {
    type Error = CommonError;

    fn try_from(value: u32) -> Result<Self> {
        if value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(CommonError::IndexOverflow)
        }
    }
}

impl HasSampleValues for U30 {
    fn sample() -> Self {
        Self::new(30)
    }
    fn sample_other() -> Self {
        Self::try_from(Self::MAX).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = U30;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample(),);
        assert_eq!(SUT::sample_other(), SUT::sample_other(),);
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other(),);
    }

    #[test]
    fn ord() {
        assert!(SUT::sample() < SUT::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<Sut>::from_iter([
                SUT::sample(),
                SUT::sample(),
                SUT::sample_other(),
                SUT::sample_other(),
            ])
            .len(),
            2
        )
    }

    #[test]
    fn try_from_valid() {
        assert_eq!(*SUT::try_from(0u32).unwrap(), 0);
        assert_eq!(*SUT::try_from(1u32).unwrap(), 1);
        assert_eq!(*SUT::try_from(SUT::MAX - 1).unwrap(), SUT::MAX - 1);
        assert_eq!(*SUT::try_from(SUT::MAX).unwrap(), SUT::MAX);
    }

    #[test]
    fn try_from_overflow() {
        assert!(SUT::try_from(SUT::MAX + 1).is_err());
    }

    #[test]
    fn add_zero() {
        let sut = SUT::new(42);
        assert_eq!(sut.checked_add(&SUT::ZERO).unwrap(), sut);
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut = SUT::try_from(SUT::MAX).unwrap();
        assert_eq!(sut.checked_add(&SUT::ZERO).unwrap(), sut,);
    }

    #[test]
    fn add_max_to_zero_is_ok() {
        let sut = SUT::ZERO;
        assert_eq!(
            sut.checked_add_n(SUT::MAX).unwrap(),
            SUT::try_from(SUT::MAX).unwrap()
        );
    }

    #[test]
    fn add_one() {
        let sut = SUT::new(42);
        assert_eq!(sut.checked_add_one().unwrap(), SUT::new(43));
    }

    #[test]
    fn add_one_to_max_minus_1_is_max() {
        let sut = SUT::try_from(SUT::MAX - 1).unwrap();
        assert_eq!(
            sut.checked_add(&SUT::ONE).unwrap(),
            SUT::try_from(SUT::MAX).unwrap()
        );
    }

    #[test]
    fn add_one_to_two() {
        assert_eq!(SUT::TWO.checked_add(&SUT::ONE).unwrap(), SUT::THREE);
    }

    #[test]
    fn addition_overflow_base_max() {
        let sut = SUT::try_from(SUT::MAX).unwrap();
        assert!(matches!(
            sut.checked_add(&SUT::ONE),
            Err(CommonError::IndexOverflow)
        ));
    }

    #[test]
    fn addition_overflow_add_max() {
        let sut = SUT::ONE;
        assert!(matches!(
            sut.checked_add(&SUT::try_from(SUT::MAX).unwrap()),
            Err(CommonError::IndexOverflow)
        ));
    }
}

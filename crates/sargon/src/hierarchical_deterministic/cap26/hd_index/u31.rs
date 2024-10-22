use crate::prelude::*;

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    derive_more::Display,
    Ord,
    derive_more::FromStr,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub struct U31(u32);

pub const U32_MAX: u32 = u32::MAX;
pub const U31_MAX: u32 = U32_MAX / 2;

impl U31 {
    pub const MAX: Self = U31(U31_MAX);
    pub const fn new(inner: u32) -> Result<Self> {
        if inner > U31_MAX {
            return Err(CommonError::InvalidU31 { bad_value: inner });
        }
        Ok(Self(inner))
    }
    fn maybe_new(inner: u32) -> Option<Self> {
        Self::new(inner).ok()
    }
}

impl Deref for U31 {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CheckedDiv for U31 {
    fn checked_div(&self, other: &Self) -> Option<Self::Output>
    where
        Self: Sized,
    {
        self.0.checked_div(**other).map(Self::maybe_new).flatten()
    }
}
impl Div for U31 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.checked_div(&other).expect("U31 division overflow")
    }
}

impl Div<u32> for U31 {
    type Output = Self;

    fn div(self, other: u32) -> Self::Output {
        self / Self::new(other).unwrap()
    }
}

impl CheckedAdd for U31 {
    fn checked_add(&self, other: &Self) -> Option<Self::Output>
    where
        Self: Sized,
    {
        self.0.checked_add(**other).map(Self::maybe_new).flatten()
    }
}
impl Add for U31 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.checked_add(&other).expect("U31 addition overflow")
    }
}

impl Add<u32> for U31 {
    type Output = Self;

    fn add(self, other: u32) -> Self::Output {
        self + Self::new(other).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = U31;

    #[test]
    fn invalid_too_large() {
        assert_eq!(
            SUT::new(U31_MAX + 1),
            Err(CommonError::InvalidU31 {
                bad_value: U31_MAX + 1
            })
        );
    }

    #[test]
    fn valid_max() {
        assert!(SUT::new(U31_MAX).is_ok())
    }

    #[test]
    fn inner() {
        assert_eq!(*SUT::new(1024).unwrap(), 1024);
    }

    #[test]
    fn ord() {
        assert!(SUT::new(0).unwrap() < SUT::new(1).unwrap());
        assert!(SUT::new(U31_MAX).unwrap() > SUT::new(U31_MAX - 1).unwrap());
    }
}

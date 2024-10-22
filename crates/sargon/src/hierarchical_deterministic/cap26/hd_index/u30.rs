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
pub struct U30(u32);

pub const U30_MAX: u32 = U31_MAX / 2;

impl U30 {
    pub const MAX: Self = U30(U30_MAX);
    pub const fn new(inner: u32) -> Result<Self> {
        if inner > U30_MAX {
            return Err(CommonError::InvalidU30 { bad_value: inner });
        }
        Ok(Self(inner))
    }
    fn maybe_new(inner: u32) -> Option<Self> {
        Self::new(inner).ok()
    }
}

impl Deref for U30 {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CheckedDiv for U30 {
    fn checked_div(&self, other: &Self) -> Option<Self::Output>
    where
        Self: Sized,
    {
        self.0.checked_div(**other).map(Self::maybe_new).flatten()
    }
}
impl Div for U30 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.checked_div(&other).expect("U31 division overflow")
    }
}

impl Div<u32> for U30 {
    type Output = Self;

    fn div(self, other: u32) -> Self::Output {
        self / Self::new(other).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = U30;

    #[test]
    fn invalid_too_large() {
        assert_eq!(
            SUT::new(U30_MAX + 1),
            Err(CommonError::InvalidU30 {
                bad_value: U30_MAX + 1
            })
        );
    }

    #[test]
    fn valid_max() {
        assert!(SUT::new(U30_MAX).is_ok())
    }

    #[test]
    fn inner() {
        assert_eq!(*SUT::new(1024).unwrap(), 1024);
    }

    #[test]
    fn ord() {
        assert!(SUT::new(0).unwrap() < SUT::new(1).unwrap());
        assert!(SUT::new(U30_MAX).unwrap() > SUT::new(U30_MAX - 1).unwrap());
    }
}

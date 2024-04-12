use crate::prelude::*;

#[derive(
    Zeroize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    derive_more::Display,
    Ord,
    Hash,
    uniffi::Record,
)]
#[display("{inner}")]
pub struct U11 {
    pub inner: u16,
}

impl U11 {
    pub const MAX: u16 = 2047;
    pub fn new(inner: u16) -> Result<Self> {
        if inner > Self::MAX {
            return Err(CommonError::InvalidBIP39Index { bad_value: inner });
        }
        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = U11;

    #[test]
    fn invalid_2048() {
        assert_eq!(
            SUT::new(2048),
            Err(CommonError::InvalidBIP39Index { bad_value: 2048 })
        );
    }

    #[test]
    fn valid_2047() {
        assert!(SUT::new(2047).is_ok())
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::new(2047).unwrap()), "2047")
    }

    #[test]
    fn inner() {
        assert_eq!(SUT::new(1024).unwrap().inner, 1024);
    }

    #[test]
    fn ord() {
        assert!(SUT::new(0).unwrap() < SUT::new(1).unwrap());
        assert!(SUT::new(2047).unwrap() > SUT::new(2046).unwrap());
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::new(1024).unwrap();
        sut.zeroize();
        assert_eq!(sut, SUT::new(0).unwrap());
    }
}

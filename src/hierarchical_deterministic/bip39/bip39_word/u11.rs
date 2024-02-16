use crate::prelude::*;

#[derive(
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
    pub fn new(inner: u16) -> Result<Self> {
        if inner >= 2048 {
            return Err(CommonError::InvalidBIP39Index { bad_value: inner });
        }
        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn invalid_2048() {
        assert_eq!(
            U11::new(2048),
            Err(CommonError::InvalidBIP39Index { bad_value: 2048 })
        );
    }

    #[test]
    fn valid_2047() {
        assert!(U11::new(2047).is_ok())
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", U11::new(2047).unwrap()), "2047")
    }

    #[test]
    fn inner() {
        assert_eq!(U11::new(1024).unwrap().inner, 1024);
    }

    #[test]
    fn ord() {
        assert!(U11::new(0).unwrap() < U11::new(1).unwrap());
        assert!(U11::new(2047).unwrap() > U11::new(2046).unwrap());
    }
}

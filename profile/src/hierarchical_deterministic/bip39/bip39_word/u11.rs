use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::CommonError;

#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, uniffi::Record,
)]
pub struct U11 {
    pub inner: u16,
}

impl U11 {
    pub fn new(inner: u16) -> Result<Self, CommonError> {
        if inner >= 2048 {
            return Err(CommonError::InvalidBIP39Index);
        }
        Ok(Self { inner })
    }
}

impl Display for U11 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::U11;
    use crate::CommonError;

    #[test]
    fn invalid_2048() {
        assert_eq!(U11::new(2048), Err(CommonError::InvalidBIP39Index));
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

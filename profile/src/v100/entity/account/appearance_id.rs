use crate::prelude::*;

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    derive_more::Display,
    PartialOrd,
    Ord,
    Hash,
    uniffi::Record,
)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
#[display("{value}")]
pub struct AppearanceID {
    pub value: u8,
}

impl AppearanceID {
    /// The number of different appearances
    pub const MAX: u8 = 11;
    pub fn new(value: u8) -> Result<Self> {
        if value > Self::MAX {
            return Err(CommonError::InvalidAppearanceID);
        }
        Ok(Self { value })
    }

    pub fn from_number_of_accounts_on_network(n: usize) -> Self {
        Self {
            value: (n % (Self::MAX as usize)) as u8,
        }
    }
}

impl Default for AppearanceID {
    fn default() -> Self {
        Self::new(0).expect("AppearanceID of 0 to be valid")
    }
}

impl TryFrom<u8> for AppearanceID {
    type Error = crate::CommonError;

    fn try_from(value: u8) -> Result<Self> {
        AppearanceID::new(value)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        assert_json_value_eq_after_roundtrip, assert_json_value_fails, AppearanceID, CommonError,
    };
    use serde_json::json;

    #[test]
    fn lowest() {
        assert!(AppearanceID::new(0).is_ok());
    }

    #[test]
    fn highest() {
        assert!(AppearanceID::new(11).is_ok());
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", AppearanceID::new(11).unwrap()), "11");
    }

    #[test]
    fn err_too_big() {
        assert_eq!(AppearanceID::new(12), Err(CommonError::InvalidAppearanceID));
    }

    #[test]
    fn try_from() {
        assert_eq!(
            AppearanceID::try_from(250),
            Err(CommonError::InvalidAppearanceID)
        );
        assert_eq!(AppearanceID::try_from(1), AppearanceID::new(1));
    }

    #[test]
    fn json() {
        assert_json_value_eq_after_roundtrip(&AppearanceID::new(3).unwrap(), json!(3));
        assert_json_value_fails::<AppearanceID>(json!("3"));
        assert_json_value_fails::<AppearanceID>(json!(99));
    }
}

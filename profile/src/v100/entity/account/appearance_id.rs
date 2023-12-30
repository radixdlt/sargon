use crate::CommonError;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(
    Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, uniffi::Record,
)]
pub struct AppearanceID {
    pub value: u8,
}

impl AppearanceID {
    pub fn new(value: u8) -> Result<Self, CommonError> {
        if value >= 11 {
            return Err(CommonError::InvalidAppearanceID);
        }
        Ok(Self { value })
    }
}

impl Display for AppearanceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Default for AppearanceID {
    fn default() -> Self {
        Self::new(0).expect("AppearanceID of 0 to be valid")
    }
}

impl TryFrom<u8> for AppearanceID {
    type Error = crate::CommonError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

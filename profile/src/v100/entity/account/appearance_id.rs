use crate::CommonError;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, uniffi::Record)]
pub struct AppearanceID {
    pub value: u8,
}


impl Serialize for AppearanceID {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.value)
    }
}

impl<'de> Deserialize<'de> for AppearanceID {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<AppearanceID, D::Error> {
        let value = u8::deserialize(d)?;
        AppearanceID::new(value).map_err(de::Error::custom)
    }
}

impl AppearanceID {
    pub fn new(value: u8) -> Result<Self, CommonError> {
        if value > 11 {
            return Err(CommonError::InvalidAppearanceID);
        }
        Ok(Self { value })
    }

    pub fn from_number_of_accounts_on_network(n: usize) -> Self {
        Self::new(n)
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

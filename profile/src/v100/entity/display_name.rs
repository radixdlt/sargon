use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::CommonError;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, uniffi::Record)]
pub struct DisplayName {
    pub value: String,
}

impl Serialize for DisplayName {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl<'de> Deserialize<'de> for DisplayName {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<DisplayName, D::Error> {
        let s = String::deserialize(d)?;
        DisplayName::new(s.to_string()).map_err(de::Error::custom)
    }
}

impl DisplayName {
    pub fn max_len() -> usize {
        30
    }

    pub fn new(value: String) -> Result<Self, CommonError> {
        let value = value.trim().to_string();
        if value.is_empty() {
            return Err(CommonError::InvalidDisplayNameEmpty);
        }
        if value.len() > Self::max_len() {
            return Err(CommonError::InvalidDisplayNameTooLong);
        }

        Ok(Self { value })
    }
}

impl Display for DisplayName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for DisplayName {
    fn default() -> Self {
        Self::new("Unnamed".to_string()).expect("Default display name")
    }
}

impl TryFrom<&str> for DisplayName {
    type Error = crate::CommonError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DisplayName::new(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip, assert_json_value_fails,
        assert_json_value_ne_after_roundtrip,
    };
    use serde_json::json;

    use super::DisplayName;
    use crate::CommonError as Error;

    #[test]
    fn invalid() {
        assert_eq!(
            DisplayName::try_from("this is a much much too long display name"),
            Err(Error::InvalidDisplayNameTooLong)
        );
    }

    #[test]
    fn max_is_ok() {
        assert!(DisplayName::try_from("0|RDX|Dev Nano S|Some very lon").is_ok());
    }

    #[test]
    fn valid_try_from() {
        assert_eq!(
            DisplayName::try_from("Main"),
            Ok(DisplayName::new("Main".to_string()).unwrap())
        );
    }

    #[test]
    fn inner() {
        assert_eq!(
            DisplayName::new("Main account".to_string()).unwrap().value,
            "Main account"
        );
    }

    #[test]
    fn json_roundtrip() {
        let a: DisplayName = "Cool persona".try_into().unwrap();

        assert_json_value_eq_after_roundtrip(&a, json!("Cool persona"));
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(&a, json!("Main account"));

        assert_json_value_fails::<DisplayName>(json!("this is a much much too long display name"));
    }
}

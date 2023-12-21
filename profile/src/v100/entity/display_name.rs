use nutype::nutype;

use wallet_kit_common::error::common_error::CommonError as Error;

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 30),
    derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Display,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )
)]
pub struct DisplayName(String);

impl Default for DisplayName {
    fn default() -> Self {
        Self::new("Unnamed").expect("Default display name")
    }
}

impl TryFrom<&str> for DisplayName {
    type Error = wallet_kit_common::error::common_error::CommonError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DisplayName::new(value.to_string()).map_err(|_| Error::InvalidDisplayName)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::assert_json::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };

    use super::DisplayName;
    use wallet_kit_common::error::common_error::CommonError as Error;

    #[test]
    fn invalid() {
        assert_eq!(
            DisplayName::try_from("this is a much much too long display name"),
            Err(Error::InvalidDisplayName)
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
            Ok(DisplayName::new("Main").unwrap())
        );
    }

    #[test]
    fn inner() {
        assert_eq!(
            DisplayName::new("Main account").unwrap().into_inner(),
            "Main account"
        );
    }

    #[test]
    fn json_roundtrip() {
        let a: DisplayName = "Cool persona".try_into().unwrap();

        assert_json_value_eq_after_roundtrip(&a, json!("Cool persona"));
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(&a, json!("Main account"));
    }
}

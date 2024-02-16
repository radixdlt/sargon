use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{value}")]
pub struct DisplayName {
    pub value: String,
}

#[uniffi::export]
pub fn new_display_name(name: String) -> Result<DisplayName> {
    DisplayName::new(name.as_str())
}

impl DisplayName {
    pub const MAX_LEN: usize = 30;

    pub fn new(value: &str) -> Result<Self> {
        let value = value.trim().to_string();
        if value.is_empty() {
            return Err(CommonError::InvalidDisplayNameEmpty);
        }
        if value.len() > Self::MAX_LEN {
            return Err(CommonError::InvalidDisplayNameTooLong {
                expected: Self::MAX_LEN as u64,
                found: value.len() as u64,
            });
        }

        Ok(Self { value })
    }
}

impl Default for DisplayName {
    fn default() -> Self {
        Self::new("Unnamed").expect("Default display name")
    }
}

impl FromStr for DisplayName {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DisplayName::new(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn invalid() {
        let s = "this is a much much too long display name";
        assert_eq!(
            DisplayName::new(s),
            Err(CommonError::InvalidDisplayNameTooLong {
                expected: DisplayName::MAX_LEN as u64,
                found: s.len() as u64
            })
        );
    }

    #[test]
    fn max_is_ok() {
        assert!(DisplayName::new("0|RDX|Dev Nano S|Some very lon").is_ok());
    }

    #[test]
    fn valid_try_from() {
        assert_eq!(
            DisplayName::new("Main"),
            Ok(DisplayName::new("Main").unwrap())
        );
    }

    #[test]
    fn empty_is_invalid() {
        assert_eq!(
            DisplayName::new(""),
            Err(CommonError::InvalidDisplayNameEmpty)
        );
    }

    #[test]
    fn spaces_trimmed_into_empty_is_invalid() {
        assert_eq!(
            DisplayName::new("   "),
            Err(CommonError::InvalidDisplayNameEmpty)
        );
    }

    #[test]
    fn inner() {
        assert_eq!(
            DisplayName::new("Main account").unwrap().value,
            "Main account"
        );
    }

    #[test]
    fn json_roundtrip() {
        let a: DisplayName = "Cool persona".parse().unwrap();

        assert_json_value_eq_after_roundtrip(&a, json!("Cool persona"));
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(&a, json!("Main account"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<DisplayName>(json!(
            "this is a much much too long display name"
        ));
        assert_json_value_fails::<DisplayName>(json!(""));
        assert_json_value_fails::<DisplayName>(json!("   "));
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{new_display_name, DisplayName};

    #[test]
    fn new() {
        assert_eq!(
            new_display_name("Main".to_string()).unwrap(),
            DisplayName::new("Main").unwrap(),
        );
    }
}

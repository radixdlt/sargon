use crate::prelude::*;

/// A max 30 chars long string used for display purposes, e.g.
/// the name of an Account or Persona.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
/// #[allow(clippy::upper_case_acronyms)]
/// type SUT = DisplayName;
///
/// assert_eq!(SUT::MAX_LEN, 30);
/// assert_eq!("Satoshi".parse::<SUT>().unwrap().to_string(), "Satoshi");
/// ```
///
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

impl HasSampleValues for DisplayName {
    fn sample() -> Self {
        "Spending Account".parse().unwrap()
    }

    fn sample_other() -> Self {
        "Savings Account".parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DisplayName;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn invalid() {
        let s = "this is a much much too long display name";
        assert_eq!(
            SUT::new(s),
            Err(CommonError::InvalidDisplayNameTooLong {
                expected: SUT::MAX_LEN as u64,
                found: s.len() as u64
            })
        );
    }

    #[test]
    fn max_is_ok() {
        assert!(SUT::new("0|RDX|Dev Nano S|Some very lon").is_ok());
    }

    #[test]
    fn valid_try_from() {
        assert_eq!(SUT::new("Main"), Ok(SUT::new("Main").unwrap()));
    }

    #[test]
    fn empty_is_invalid() {
        assert_eq!(SUT::new(""), Err(CommonError::InvalidDisplayNameEmpty));
    }

    #[test]
    fn spaces_trimmed_into_empty_is_invalid() {
        assert_eq!(SUT::new("   "), Err(CommonError::InvalidDisplayNameEmpty));
    }

    #[test]
    fn inner() {
        assert_eq!(SUT::new("Main account").unwrap().value, "Main account");
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT = "Cool persona".parse().unwrap();

        assert_json_value_eq_after_roundtrip(&a, json!("Cool persona"));
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(&a, json!("Main account"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(
            "this is a much much too long display name"
        ));
        assert_json_value_fails::<SUT>(json!(""));
        assert_json_value_fails::<SUT>(json!("   "));
    }
}

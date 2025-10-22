use crate::prelude::*;

/// A max 255 chars string which impl Copy
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
)]
#[display("{}", self.value())]
pub struct ShortString(arraystring::MaxString);

impl ShortString {
    pub fn value(&self) -> String {
        self.0.clone().to_string()
    }

    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        arraystring::MaxString::try_from_str(value.as_ref())
            .map_err(|_| CommonError::Unknown {
                error_message: "Unable to init MaxString".to_string(),
            })
            .map(Self)
    }
}

impl FromStr for ShortString {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ShortString::new(s)
    }
}

impl HasSampleValues for ShortString {
    fn sample() -> Self {
        "Hello World".parse().unwrap()
    }

    fn sample_other() -> Self {
        "The human spirit must prevail over technology"
            .parse()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ShortString;

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
    fn max_is_ok() {
        assert!(SUT::new("0|RDX|Dev Nano S|Some very lon").is_ok());
    }

    #[test]
    fn valid_try_from() {
        assert_eq!(SUT::new("Main"), Ok(SUT::new("Main").unwrap()));
    }

    #[test]
    fn inner() {
        assert_eq!(SUT::new("Main account").unwrap().value(), "Main account");
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
        assert_json_value_fails::<SUT>(json!("this is a much much too long string which character count way exceed the limit which is two hundred and fifty five or written with digits 255 which is u8 max value which is 2^8 - 1 and it is in fact quite a long string it allows for, but this is just too long."));
    }
}

use crate::prelude::*;

/// Path used to respond to dapp requests in Mobile Connect flow. Read from well known file.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::FromStr,
    derive_more::Display,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct RCMCallbackPath(pub(crate) String);

impl RCMCallbackPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<&str> for RCMCallbackPath {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Default for RCMCallbackPath {
    fn default() -> Self {
        Self::new("connect")
    }
}

impl HasSampleValues for RCMCallbackPath {
    fn sample() -> Self {
        Self::new("sample_callback_path")
    }

    fn sample_other() -> Self {
        Self::new("sample_other_callback_path")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RCMCallbackPath;

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
    fn from_string() {
        let value = "from_string";
        let callback_path = RCMCallbackPath::from(value);
        assert_eq!(callback_path, RCMCallbackPath(value.to_owned()));
    }

    #[test]
    fn test_default() {
        assert_eq!(
            RCMCallbackPath::default(),
            RCMCallbackPath("connect".to_owned())
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut: SUT = "/connect".into();
        assert_json_value_eq_after_roundtrip(&sut, json!("/connect"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("foobar"));
    }
}

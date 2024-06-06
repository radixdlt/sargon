use crate::prelude::*;

/// Path used to respond to dapp requests in Mobile Connect flow. Read from well known file.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::FromStr,
    derive_more::Display,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub struct RCMCallbackPath(pub(crate) String);

impl RCMCallbackPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<String> for RCMCallbackPath {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Default for RCMCallbackPath {
    fn default() -> Self {
        Self("default_callback_path".to_owned())
    }
}

impl HasSampleValues for RCMCallbackPath {
    fn sample() -> Self {
        Self("sample_callback_path".to_owned())
    }

    fn sample_other() -> Self {
        Self("sample_callback_path_other".to_owned())
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
        let value = "from_string".to_owned();
        let callback_path = RCMCallbackPath::from(value.clone());
        assert_eq!(callback_path, RCMCallbackPath(value));
    }

    #[test]
    fn test_default() {
        assert_eq!(
            RCMCallbackPath::default(),
            RCMCallbackPath("default_callback_path".to_owned())
        );
    }
}

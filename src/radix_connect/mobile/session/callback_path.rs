use crate::prelude::*;

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
}

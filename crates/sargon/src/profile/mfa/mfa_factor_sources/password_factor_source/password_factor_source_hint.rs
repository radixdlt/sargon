use crate::prelude::*;

/// Properties describing a PasswordFactorSource to help user disambiguate between
/// it and another one.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
pub struct PasswordFactorSourceHint {
    pub label: String,
}

impl PasswordFactorSourceHint {
    pub fn new(label: impl AsRef<str>) -> Self {
        Self {
            label: label.as_ref().to_owned(),
        }
    }
}

impl HasSampleValues for PasswordFactorSourceHint {
    fn sample() -> Self {
        Self::new("Password 1")
    }

    fn sample_other() -> Self {
        Self::new("Password 2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PasswordFactorSourceHint;

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

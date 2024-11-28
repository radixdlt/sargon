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
    pub display_name: DisplayName,
}

impl PasswordFactorSourceHint {
    pub fn new(display_name: DisplayName) -> Self {
        Self { display_name }
    }
}

impl HasSampleValues for PasswordFactorSourceHint {
    fn sample() -> Self {
        Self::new(DisplayName::new("Password 1").unwrap())
    }

    fn sample_other() -> Self {
        Self::new(DisplayName::new("Password 2").unwrap())
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

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FactorRolesValidation {
    /// Skips validation completely
    Skip,

    /// Perform validation on the factors
    Validate {
        /// If `allow_not_yet_valid` we map `NotYetValid` to `Ok(())`
        allow_not_yet_valid: bool,
    },
}

impl HasSampleValues for FactorRolesValidation {
    fn sample() -> Self {
        FactorRolesValidation::Skip
    }
    fn sample_other() -> Self {
        FactorRolesValidation::Validate {
            allow_not_yet_valid: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorRolesValidation;

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

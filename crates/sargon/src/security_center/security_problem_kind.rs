use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
/// An enum describing the different types of Security Problems the Wallet can encounter.
pub enum SecurityProblemKind {
    SecurityFactors,

    ConfigurationBackup,
}

impl HasSampleValues for SecurityProblemKind {
    fn sample() -> Self {
        Self::SecurityFactors
    }

    fn sample_other() -> Self {
        Self::ConfigurationBackup
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityProblemKind;

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

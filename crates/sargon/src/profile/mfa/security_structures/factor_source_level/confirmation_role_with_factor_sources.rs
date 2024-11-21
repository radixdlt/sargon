use crate::prelude::*;

impl HasSampleValues for ConfirmationRoleWithFactorSources {
    fn sample() -> Self {
        Self::new(
            [],
            0,
            [
                FactorSource::sample_security_questions(),
                FactorSource::sample_ledger(),
            ],
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            [],
            0,
            [
                FactorSource::sample_security_questions_other(),
                FactorSource::sample_ledger_other(),
            ],
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ConfirmationRoleWithFactorSources;

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

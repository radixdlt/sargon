use crate::prelude::*;

impl HasSampleValues for PrimaryRoleWithFactorSources {
    fn sample() -> Self {
        Self::new(
            [
                FactorSource::sample_device_babylon(),
                FactorSource::sample_arculus(),
                FactorSource::sample_off_device(),
            ],
            2,
            [FactorSource::sample_ledger()],
        )
        .unwrap()
    }
    fn sample_other() -> Self {
        Self::new(
            [
                FactorSource::sample_device_babylon_other(),
                FactorSource::sample_arculus_other(),
                FactorSource::sample_off_device_other(),
            ],
            2,
            [FactorSource::sample_ledger_other()],
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PrimaryRoleWithFactorSources;

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

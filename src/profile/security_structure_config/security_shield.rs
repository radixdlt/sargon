use crate::prelude::*;

decl_security_shield_at_level!(
    /// A "Schematic Of SecurityShield", is at `FactorSourceID` level.
    /// This is what is saved into Profile, being just references to FactorSources.
    /// The user views, creates and manages SecurityShields, which is at `FactorSource`
    /// level.
    SecurityShield,
    FactorSource,
);

impl Identifiable for SecurityShield {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.metadata.id()
    }
}

impl HasSampleValues for SecurityShieldPrimaryRole {
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
    }
}

impl HasSampleValues for SecurityShieldRecoveryRole {
    fn sample() -> Self {
        Self::new(
            [
                FactorSource::sample_trusted_contact_frank(),
                FactorSource::sample_trusted_contact_grace(),
                FactorSource::sample_trusted_contact_judy(),
            ],
            2,
            [FactorSource::sample_ledger()],
        )
    }
    fn sample_other() -> Self {
        Self::new(
            [
                FactorSource::sample_trusted_contact_trudy(),
                FactorSource::sample_trusted_contact_oscar(),
                FactorSource::sample_trusted_contact_radix(),
            ],
            2,
            [FactorSource::sample_ledger_other()],
        )
    }
}

impl HasSampleValues for SecurityShieldConfirmationRole {
    fn sample() -> Self {
        Self::new(
            [],
            0,
            [
                FactorSource::sample_security_questions(),
                FactorSource::sample_ledger(),
            ],
        )
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
    }
}

impl HasSampleValues for SecurityShield {
    fn sample() -> Self {
        Self::new(
            SecurityStructureMetadata::sample(),
            SecurityShieldConfiguration::new(
                SecurityShieldPrimaryRole::sample(),
                SecurityShieldRecoveryRole::sample(),
                SecurityShieldConfirmationRole::sample(),
                4096, // 14.2 days
            ),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            SecurityStructureMetadata::sample_other(),
            SecurityShieldConfiguration::new(
                SecurityShieldPrimaryRole::sample_other(),
                SecurityShieldRecoveryRole::sample_other(),
                SecurityShieldConfirmationRole::sample_other(),
                8192, // 28.4 days
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShield;

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
    fn test_into_schematics_and_back() {
        let factor_sources = FactorSources::sample_values_all();
        let sut = SUT::sample();
        let schematics = SchematicOfSecurityShield::from(sut.clone());
        let detailed = SUT::try_from((&schematics, &factor_sources)).unwrap();
        assert_eq!(detailed, sut);
    }
}

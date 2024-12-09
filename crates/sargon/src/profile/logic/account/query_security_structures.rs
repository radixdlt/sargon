use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SecurityStructureOfFactorSources`]
    SecurityStructuresOfFactorSources,
    SecurityStructureOfFactorSources
);

impl HasSampleValues for SecurityStructuresOfFactorSources {
    fn sample() -> Self {
        Self::from_iter([
            SecurityStructureOfFactorSources::sample(),
            SecurityStructureOfFactorSources::sample_other(),
        ])
    }
    fn sample_other() -> Self {
        Self::from_iter([SecurityStructureOfFactorSources::sample_other()])
    }
}

impl Profile {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    pub fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources> {
        self.app_preferences
            .security
            .security_structures_of_factor_source_ids
            .iter()
            .map(|id| {
                SecurityStructureOfFactorSources::try_from((
                    &id,
                    &self.factor_sources,
                ))
            })
            .collect::<Result<SecurityStructuresOfFactorSources>>()
    }
}

impl Profile {
    /// Returns the status of the prerequisites for building a Security Shield.
    ///
    /// According to [definition][doc], a Security Shield can be built if the user has, asides from
    /// the Identity factor, "2 or more factors, one of which must be Hardware"
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Factor-Prerequisites
    pub fn security_shield_prerequisites_status(
        &self,
    ) -> SecurityShieldPrerequisitesStatus {
        let factor_source_ids = self
            .factor_sources
            .iter()
            .map(|f| f.id())
            .collect::<IndexSet<_>>();
        SecurityShieldBuilder::prerequisites_status(&factor_source_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn security_shield_prerequisites_status_hardware_required() {
        let mut sut = SUT::sample();

        // Test the case where user doesn't have any factors
        sut.factor_sources = FactorSources::from_iter([]);
        let result = sut.security_shield_prerequisites_status();
        assert_eq!(result, SecurityShieldPrerequisitesStatus::HardwareRequired);

        // Test the case where the user has identity factor
        sut.factor_sources =
            FactorSources::from_iter([FactorSource::sample_device()]);
        let result = sut.security_shield_prerequisites_status();
        assert_eq!(result, SecurityShieldPrerequisitesStatus::HardwareRequired);

        // Test the case where the user also has other non-hardware factors
        sut.factor_sources = FactorSources::from_iter([
            FactorSource::sample_device(),
            FactorSource::sample_password(),
            FactorSource::sample_trusted_contact_frank(),
            FactorSource::sample_off_device(),
        ]);
        let result = sut.security_shield_prerequisites_status();
        assert_eq!(result, SecurityShieldPrerequisitesStatus::HardwareRequired);
    }

    #[test]
    fn security_shield_prerequisites_status_any_required() {
        let mut sut = SUT::sample();

        // Test the case where user only has hardware factor
        sut.factor_sources =
            FactorSources::from_iter([FactorSource::sample_arculus()]);
        let result = sut.security_shield_prerequisites_status();
        assert_eq!(result, SecurityShieldPrerequisitesStatus::AnyRequired);

        // Test the case where the user also has identity factors
        sut.factor_sources = FactorSources::from_iter([
            FactorSource::sample_arculus(),
            FactorSource::sample_device(),
        ]);
        let result = sut.security_shield_prerequisites_status();
        assert_eq!(result, SecurityShieldPrerequisitesStatus::AnyRequired);
    }

    #[test]
    fn security_shield_prerequisites_status_sufficient() {
        let mut sut = SUT::sample();

        // Test the case where user only has hardware factors
        sut.factor_sources = FactorSources::from_iter([
            FactorSource::sample_arculus(),
            FactorSource::sample_ledger(),
        ]);
        let result = sut.security_shield_prerequisites_status();
        assert_eq!(result, SecurityShieldPrerequisitesStatus::Sufficient);

        // Test the case where the user has 1 hardware factor and 1 non-hardware factor
        sut.factor_sources = FactorSources::from_iter([
            FactorSource::sample_ledger(),
            FactorSource::sample_password(),
        ]);
        let result = sut.security_shield_prerequisites_status();
        assert_eq!(result, SecurityShieldPrerequisitesStatus::Sufficient);
    }
}

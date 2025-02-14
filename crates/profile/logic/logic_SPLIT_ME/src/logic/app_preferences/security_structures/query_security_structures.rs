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

pub trait ProfileSecurityStructuresOfFactorSources {
    fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources>;
}

impl ProfileSecurityStructuresOfFactorSources for Profile {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    fn security_structures_of_factor_sources(
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

pub trait ProfileSecurityShieldPrerequisitesStatus {
    fn security_shield_prerequisites_status(
        &self,
    ) -> SecurityShieldPrerequisitesStatus;
}

impl ProfileSecurityShieldPrerequisitesStatus for Profile {
    /// Returns the status of the prerequisites for building a Security Shield.
    ///
    /// According to [definition][doc], a Security Shield can be built if the user has, asides from
    /// the Identity factor, "2 or more factors, one of which must be Hardware"
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Factor-Prerequisites
    fn security_shield_prerequisites_status(
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

pub trait LinkedToSecurityStructure {
    fn is_currently_or_provisionally_securified_with(
        &self,
        shield_id: SecurityStructureID,
    ) -> bool;

    fn is_currently_securified_with(
        &self,
        shield_id: SecurityStructureID,
    ) -> bool;

    fn is_provisionally_securified_with(
        &self,
        shield_id: SecurityStructureID,
    ) -> bool;
}

impl LinkedToSecurityStructure for EntitySecurityState {
    fn is_currently_or_provisionally_securified_with(
        &self,
        shield_id: SecurityStructureID,
    ) -> bool {
        self.is_currently_securified_with(shield_id)
            || self.is_provisionally_securified_with(shield_id)
    }

    fn is_currently_securified_with(
        &self,
        shield_id: SecurityStructureID,
    ) -> bool {
        self.as_securified()
            .map(|s| s.security_structure.security_structure_id == shield_id)
            .unwrap_or(false)
    }

    fn is_provisionally_securified_with(
        &self,
        shield_id: SecurityStructureID,
    ) -> bool {
        self.get_provisional()
            .as_ref()
            .and_then(|p| p.as_factor_instances_derived())
            .map(|s| s.security_structure_id == shield_id)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn test_account_is_currently_securified() {
        let mut sut = SUT::sample();
        let security_structure_sample =
            SecurityStructureOfFactorSourceIDs::sample();

        let accounts = sut.accounts_on_current_network().unwrap();
        let account = accounts.first().unwrap();
        let account_secured_control = SecuredEntityControl::new(
            account
                .security_state()
                .as_unsecured()
                .unwrap()
                .transaction_signing
                .clone(),
            AccessControllerAddress::sample_mainnet(),
            SecurityStructureOfFactorInstances::sample(),
        )
        .unwrap();

        let mut securify_account = account.clone();
        securify_account
            .set_security_state(EntitySecurityState::Securified {
                value: account_secured_control,
            })
            .unwrap();

        sut.update_entities_erased(
            vec![AccountOrPersona::from(securify_account)].into(),
        )
        .unwrap();

        let updated_accounts = sut.accounts_on_current_network().unwrap();
        let updated_account = updated_accounts.first().unwrap();

        let account_security_state = updated_account.security_state();

        let is_currently_securified = account_security_state
            .is_currently_securified_with(security_structure_sample.id());
        assert!(is_currently_securified);

        let is_provisionally_securified = account_security_state
            .is_provisionally_securified_with(security_structure_sample.id());
        assert!(!is_provisionally_securified);
    }

    #[test]
    fn test_account_is_provisionally_securified() {
        let mut sut = SUT::sample();
        let security_structure_sample =
            SecurityStructureOfFactorSourceIDs::sample();

        let accounts = sut.accounts_on_current_network().unwrap();
        let account = accounts.first().unwrap();
        let account_secured_control = UnsecuredEntityControl::new(
            account
                .security_state()
                .as_unsecured()
                .unwrap()
                .transaction_signing
                .clone(),
            ProvisionalSecurifiedConfig::sample(),
        )
        .unwrap();

        let mut securify_account = account.clone();
        securify_account
            .set_security_state(EntitySecurityState::Unsecured {
                value: account_secured_control,
            })
            .unwrap();

        sut.update_entities_erased(
            vec![AccountOrPersona::from(securify_account)].into(),
        )
        .unwrap();

        let updated_accounts = sut.accounts_on_current_network().unwrap();
        let updated_account = updated_accounts.first().unwrap();

        let account_security_state = updated_account.security_state();

        let is_currently_securified = account_security_state
            .is_currently_securified_with(security_structure_sample.id());
        assert!(!is_currently_securified);

        let is_provisionally_securified = account_security_state
            .is_provisionally_securified_with(security_structure_sample.id());
        assert!(is_provisionally_securified);
    }

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

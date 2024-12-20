use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    pub fn security_structures_of_factor_sources(
        &self,
    ) -> Result<Vec<SecurityStructureOfFactorSources>> {
        self.wrapped
            .security_structures_of_factor_sources()
            .into_result()
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    pub fn security_structures_of_factor_source_ids(
        &self,
    ) -> Result<Vec<SecurityStructureOfFactorSourceIDs>> {
        self.wrapped
            .security_structures_of_factor_source_ids()
            .into_result()
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    pub fn security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(
        &self,
        structure_of_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<SecurityStructureOfFactorSources> {
        self.wrapped.security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(&structure_of_ids.into_internal()).into_result()
    }

    /// Adds the security structureof factor sources to Profile if none with the
    /// same ID already exists, and if all factors it references are found in Profile.
    ///
    /// If `structure` references a FactorSource by ID which is unknown to Profile,
    /// `Err(CommonError::StructureReferencesUnknownFactorSource)` is returned.
    ///
    /// If Profile already contains a structure with the same ID, `Ok(false)` is
    /// returned **without** modifying the existing one.
    ///  
    /// # Emits Events
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::SecurityStructureAdded { id } }`
    pub async fn add_security_structure_of_factor_sources(
        &self,
        structure: &SecurityStructureOfFactorSources,
    ) -> Result<bool> {
        self.wrapped
            .add_security_structure_of_factor_sources(
                &structure.into_internal(),
            )
            .await
            .into_result()
    }

    /// Returns the status of the prerequisites for building a Security Shield.
    ///
    /// According to [definition][doc], a Security Shield can be built if the user has, asides from
    /// the Identity factor, "2 or more factors, one of which must be Hardware"
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Factor-Prerequisites
    pub fn security_shield_prerequisites_status(
        &self,
    ) -> Result<SecurityShieldPrerequisitesStatus> {
        self.wrapped
            .security_shield_prerequisites_status()
            .into_result()
    }
}

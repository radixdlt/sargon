use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    pub fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources> {
        map_result_from_internal(self.wrapped.security_structures_of_factor_sources())
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    pub fn security_structures_of_factor_source_ids(
        &self,
    ) -> Result<SecurityStructuresOfFactorSourceIDs> {
        map_result_from_internal(self.wrapped.security_structures_of_factor_source_ids())
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    pub fn security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(
        &self,
        structure_of_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<SecurityStructureOfFactorSources> {
        map_result_from_internal(self.wrapped.security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(structure_of_ids))
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
        map_result_from_internal(self.wrapped.add_security_structure_of_factor_sources(structure).await)
    }
}
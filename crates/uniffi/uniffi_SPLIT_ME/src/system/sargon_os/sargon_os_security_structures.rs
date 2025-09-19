use sargon::{
    OsSecurityShieldPrerequisiteStatus, OsSecurityStructuresQuerying,
};

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
            .into_iter_result()
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    pub fn security_structures_of_factor_source_ids(
        &self,
    ) -> Result<Vec<SecurityStructureOfFactorSourceIDs>> {
        self.wrapped
            .security_structures_of_factor_source_ids()
            .into_iter_result()
    }

    /// Returns the `SecurityStructureOfFactorSourceIDs` with the given `shield_id`.
    pub fn security_structure_of_factor_source_ids_by_security_structure_id(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<SecurityStructureOfFactorSourceIDs> {
        self.wrapped
            .security_structure_of_factor_source_ids_by_security_structure_id(
                shield_id.into_internal(),
            )
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

    /// Adds the security structure of factor source IDs to Profile if none with the
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
    pub async fn add_security_structure_of_factor_source_ids(
        &self,
        structure_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<()> {
        self.wrapped
            .add_security_structure_of_factor_source_ids(
                &structure_ids.into_internal(),
            )
            .await
            .into_result()
    }

    /// Sets the Security Shield with the given `shield_id` as the main shield.
    /// If a main Security Shield already exists, it is removed and replaced with the new one.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::SecurityStructuresUpdated { id } }`
    pub async fn set_main_security_structure(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<()> {
        self.wrapped
            .set_main_security_structure(shield_id.into_internal())
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

    /// Returns all the Security Shields along with the number of entities linked to each Security Shield,
    /// either provisionally or currently securified.
    pub async fn get_shields_for_display(
        &self,
    ) -> Result<Vec<ShieldForDisplay>> {
        self.wrapped
            .get_shields_for_display()
            .await
            .into_iter_result()
    }

    /// Returns the `SecurityStructuresOfFactorSources` based on the security state of the
    /// account or persona with given `address_of_account_or_persona`
    pub fn security_structure_of_factor_sources_from_address_of_account_or_persona(
        &self,
        address_of_account_or_persona: &AddressOfAccountOrPersona,
    ) -> Result<SecurityStructureOfFactorSources> {
        self.wrapped.security_structure_of_factor_sources_from_address_of_account_or_persona(&address_of_account_or_persona.into_internal()).into_result()
    }
}

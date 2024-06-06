use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    pub fn security_structures_of_factor_source_ids(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources> {
        self.profile_holder.access_profile_with(|p| {
            p.security_structures_of_factor_source_ids()
        })
    }

    /// Adds the security stricture of factor sources to Profile if none with the
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
        let id = structure.id();
        let structure_id_level =
            SecurityStructureOfFactorSourceIDs::from(structure.clone());

        let ids_of_factors_in_profile = self.factor_source_ids();
        let ids_in_structure = structure_id_level
            .all_factors()
            .into_iter()
            .cloned()
            .collect::<HashSet<FactorSourceID>>();

        let factors_only_in_structure =
            ids_in_structure.difference(&ids_of_factors_in_profile);
        // If `structure` references factors by ID which are not present in Profile
        let has_unknown_factors =
            !factors_only_in_structure.collect_vec().is_empty();

        if has_unknown_factors {
            return Err(CommonError::StructureReferencesUnknownFactorSource);
        }

        let inserted = self
            .update_profile_with(|mut p| {
                Ok(p.app_preferences
                    .security
                    .security_structures_of_factor_source_ids
                    .append(structure_id_level.clone())
                    .0)
            })
            .await?;

        if inserted {
            self.event_bus
                .emit(EventNotification::profile_modified(
                    EventProfileModified::SecurityStructureAdded { id },
                ))
                .await;
        }
        Ok(inserted)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn add_structure() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        for fs in FactorSources::sample_values_all().into_iter() {
            os.add_factor_source(fs).await.unwrap();
        }

        // ACT
        let structure_factor_source_level =
            SecurityStructureOfFactorSources::sample();
        let inserted = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_sources(
                    &structure_factor_source_level,
                )
            })
            .await
            .unwrap();

        let structure_factor_id_level =
            SecurityStructureOfFactorSourceIDs::from(
                structure_factor_source_level.clone(),
            );

        // ASSERT
        assert!(inserted);
        assert!(os
            .profile()
            .app_preferences
            .security
            .security_structures_of_factor_source_ids
            .contains_by_id(&structure_factor_id_level));

        let structures = os
            .security_structures_of_factor_source_ids()
            .unwrap()
            .items();

        assert!(structures.contains(&structure_factor_source_level));
    }

    #[actix_rt::test]
    async fn when_adding_structure_referencing_unknown_factors_error_is_thrown()
    {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let structure = SecurityStructureOfFactorSources::sample();
        let res = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_sources(&structure)
            })
            .await;

        // ASSERT
        assert_eq!(
            res,
            Err(CommonError::StructureReferencesUnknownFactorSource)
        );
    }
}

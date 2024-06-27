use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    pub fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources> {
        self.profile_holder
            .access_profile_with(|p| p.security_structures_of_factor_sources())
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    pub fn security_structures_of_factor_source_ids(
        &self,
    ) -> SecurityStructuresOfFactorSourceIDs {
        self.profile_holder.access_profile_with(|p| {
            p.app_preferences
                .security
                .security_structures_of_factor_source_ids
                .clone()
        })
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    pub fn security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(
        &self,
        structure_of_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<SecurityStructureOfFactorSources> {
        self.profile_holder.try_access_profile_with(|p| {
            SecurityStructureOfFactorSources::try_from((
                structure_of_ids,
                &p.factor_sources,
            ))
        })
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

        os.with_timeout(|x| x.debug_add_all_sample_factors())
            .await
            .unwrap();

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

        let structures =
            os.security_structures_of_factor_sources().unwrap().items();

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

    #[actix_rt::test]
    async fn add_when_failed_to_add_structure_no_security_structure_related_event_is_emitted(
    ) {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        // ACT
        let structure = SecurityStructureOfFactorSources::sample();
        let res = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_sources(&structure)
            })
            .await;

        // ASSERT
        assert!(res.is_err());
        assert!(!event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::SecurityStructureAdded));
    }

    #[actix_rt::test]
    async fn add_structure_emits_event() {
        // ARRANGE
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        os.with_timeout(|x| x.debug_add_all_sample_factors())
            .await
            .unwrap();

        // ACT
        let structure = SecurityStructureOfFactorSources::sample();
        let id = structure.metadata.id;
        let inserted = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_sources(&structure)
            })
            .await
            .unwrap();

        // ASSERT
        assert!(inserted);
        assert!(event_bus_driver.recorded().iter().any(|e| e.event
            == Event::ProfileModified {
                change: EventProfileModified::SecurityStructureAdded { id }
            }));
    }

    #[actix_rt::test]
    async fn get_structure_from_id() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        for fs in FactorSources::sample_values_all().into_iter() {
            os.add_factor_source(fs).await.unwrap();
        }

        // ACT
        let structure_source_sample =
            SecurityStructureOfFactorSources::sample();
        let structure_source_sample_other =
            SecurityStructureOfFactorSources::sample_other();
        let inserted = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_sources(
                    &structure_source_sample,
                )
            })
            .await
            .unwrap();
        assert!(inserted);

        let inserted = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_sources(
                    &structure_source_sample_other,
                )
            })
            .await
            .unwrap();
        assert!(inserted);

        let structure_id_sample = SecurityStructureOfFactorSourceIDs::from(
            structure_source_sample.clone(),
        );
        let structure_id_sample_other =
            SecurityStructureOfFactorSourceIDs::from(
                structure_source_sample_other.clone(),
            );

        // ASSERT
        assert_eq!(
            os.security_structures_of_factor_source_ids(),
            SecurityStructuresOfFactorSourceIDs::from_iter([
                structure_id_sample.clone(),
                structure_id_sample_other.clone(),
            ])
        );

        let structures = os.security_structures_of_factor_sources().unwrap();

        let sample_by_lookup = os.security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(
                &structure_id_sample,
            ).unwrap();

        let sample_by_lookup_other =
        os.security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(
            &structure_id_sample_other,
        ).unwrap();

        let sources_by_id_lookup =
            SecurityStructuresOfFactorSources::from_iter([
                sample_by_lookup,
                sample_by_lookup_other,
            ]);

        assert_eq!(sources_by_id_lookup, structures);
    }
}

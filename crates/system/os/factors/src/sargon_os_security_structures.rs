use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsSecurityStructuresQuerying {
    fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources>;

    fn security_structures_of_factor_source_ids(
        &self,
    ) -> Result<SecurityStructuresOfFactorSourceIDs>;

    fn security_structure_of_factor_source_ids_from_security_structure_id(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<SecurityStructureOfFactorSourceIDs>;

    fn security_structure_of_factor_sources_from_security_structure_id(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<SecurityStructureOfFactorSources> {
        let shield_id_level = self
            .security_structure_of_factor_source_ids_from_security_structure_id(
                shield_id,
            )?;
        self.security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(&shield_id_level)
    }

    fn security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(
        &self,
        structure_of_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<SecurityStructureOfFactorSources>;

    async fn add_security_structure_of_factor_source_ids(
        &self,
        structure_of_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<bool> {
        let shield_of_sources = self.security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(&structure_of_ids)?;
        self.add_security_structure_of_factor_sources(&shield_of_sources)
            .await
    }

    async fn add_security_structure_of_factor_sources(
        &self,
        structure: &SecurityStructureOfFactorSources,
    ) -> Result<bool>;
}

#[async_trait::async_trait]
impl OsSecurityStructuresQuerying for SargonOS {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources> {
        self.profile()
            .and_then(|p| p.security_structures_of_factor_sources())
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    fn security_structures_of_factor_source_ids(
        &self,
    ) -> Result<SecurityStructuresOfFactorSourceIDs> {
        self.profile().map(|p| {
            p.app_preferences
                .security
                .security_structures_of_factor_source_ids
                .clone()
        })
    }

    /// Returns all the `SecurityStructuresOfFactorSourceIDs` which are stored
    /// in profile.
    fn security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(
        &self,
        structure_of_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<SecurityStructureOfFactorSources> {
        self.profile().and_then(|p| {
            SecurityStructureOfFactorSources::try_from((
                structure_of_ids,
                &p.factor_sources,
            ))
        })
    }

    fn security_structure_of_factor_source_ids_from_security_structure_id(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<SecurityStructureOfFactorSourceIDs> {
        self.profile().and_then(|p| {
            p.app_preferences
                .security
                .security_structures_of_factor_source_ids
                .get_id(shield_id)
                .ok_or(CommonError::UnknownSecurityStructureID {
                    id: shield_id.to_string(),
                })
                .cloned()
        })
    }

    /// Adds the `SecurityStructureOfFactorSources` to Profile if none with the
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
    async fn add_security_structure_of_factor_sources(
        &self,
        structure: &SecurityStructureOfFactorSources,
    ) -> Result<bool> {
        let id = structure.id();
        let structure_id_level =
            SecurityStructureOfFactorSourceIDs::from(structure.clone());

        let ids_of_factors_in_profile = self.factor_source_ids()?;
        let ids_in_structure = structure_id_level
            .all_factors()
            .into_iter()
            .cloned()
            .collect::<HashSet<FactorSourceID>>();

        let factors_only_in_structure =
            ids_in_structure.difference(&ids_of_factors_in_profile);

        // If `structure` references factors by ID which are not present in Profile
        let ids_of_missing_factors = factors_only_in_structure.collect_vec();

        if let Some(unknown_factor_source_id) = ids_of_missing_factors.first() {
            return Err(CommonError::StructureReferencesUnknownFactorSource {
                bad_value: unknown_factor_source_id.to_string(),
            });
        }

        let inserted = self
            .update_profile_with(|p| {
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

pub trait OsSecurityShieldPrerequisiteStatus {
    fn security_shield_prerequisites_status(
        &self,
    ) -> Result<SecurityShieldPrerequisitesStatus>;
}

impl OsSecurityShieldPrerequisiteStatus for SargonOS {
    /// Returns the status of the prerequisites for building a Security Shield.
    ///
    /// According to [definition][doc], a Security Shield can be built if the user has, asides from
    /// the Identity factor, "2 or more factors, one of which must be Hardware"
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Factor-Prerequisites
    fn security_shield_prerequisites_status(
        &self,
    ) -> Result<SecurityShieldPrerequisitesStatus> {
        self.profile()
            .map(|p| p.security_shield_prerequisites_status())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn add_structure() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        assert_eq!(
            FactorSource::sample_security_questions().id_from_hash(),
            FactorSourceIDFromHash::sample_security_questions()
        );

        assert_eq!(
            FactorSource::sample_security_questions_other().id_from_hash(),
            FactorSourceIDFromHash::sample_security_questions_other()
        );

        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
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
            .unwrap()
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
        assert!(matches!(
            res,
            Err(CommonError::StructureReferencesUnknownFactorSource {
                bad_value: _
            })
        ));
    }

    #[actix_rt::test]
    async fn add_when_failed_to_add_structure_no_security_structure_related_event_is_emitted(
    ) {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let mut clients = Clients::new(Bios::new(drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
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
        let mut clients = Clients::new(Bios::new(drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();
        os.with_timeout(|x| x.new_wallet(false)).await.unwrap();

        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
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

        for fs in FactorSources::sample_values_all_hd().into_iter() {
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
            os.security_structures_of_factor_source_ids().unwrap(),
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

    #[actix_rt::test]
    async fn security_shield_prerequisites_status() {
        let os = SUT::fast_boot().await;
        let result = os.security_shield_prerequisites_status().unwrap();
        assert_eq!(result, SecurityShieldPrerequisitesStatus::HardwareRequired);
    }
}

use crate::prelude::*;
use futures::future::join_all;

#[async_trait::async_trait]
pub trait OsSecurityStructuresQuerying {
    fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources>;

    fn security_structures_of_factor_source_ids(
        &self,
    ) -> Result<SecurityStructuresOfFactorSourceIDs>;

    fn security_structure_of_factor_source_ids_by_security_structure_id(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<SecurityStructureOfFactorSourceIDs>;

    fn security_structure_of_factor_sources_from_security_structure_id(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<SecurityStructureOfFactorSources> {
        let shield_id_level = self
            .security_structure_of_factor_source_ids_by_security_structure_id(
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
        structure_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<()>;

    async fn update_security_structure_of_factor_source_ids(
        &self,
        structure_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<()>;

    async fn set_main_security_structure(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<()>;

    async fn entities_linked_to_security_structure(
        &self,
        shield_id: SecurityStructureID,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesLinkedToSecurityStructure>;

    async fn get_shields_for_display(&self) -> Result<ShieldsForDisplay>;

    async fn rename_security_structure(
        &self,
        shield_id: SecurityStructureID,
        name: DisplayName,
    ) -> Result<()>;

    fn security_structure_of_factor_sources_from_address_of_account_or_persona(
        &self,
        address_of_account_or_persona: &AddressOfAccountOrPersona,
    ) -> Result<SecurityStructureOfFactorSources>;

    fn provisional_security_structure_of_factor_sources_from_address_of_account_or_persona(
        &self,
        address_of_account_or_persona: &AddressOfAccountOrPersona,
    ) -> Result<SecurityStructureOfFactorSources>;

    fn sorted_factor_sources_from_security_structure(
        &self,
        structure: &SecurityStructureOfFactorSources,
    ) -> IndexSet<FactorSource>;
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

    fn security_structure_of_factor_source_ids_by_security_structure_id(
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

    /// Adds the `SecurityStructureOfFactorSourceIDs` to Profile if none with the
    /// same ID already exists, and if all factors it references are found in Profile.
    ///
    /// If `structure` references a FactorSource by ID which is unknown to Profile,
    /// `Err(CommonError::StructureReferencesUnknownFactorSource)` is returned.
    ///
    /// If Profile already contains a structure with the same ID, `Err(CommonError::)` is
    /// returned **without** modifying the existing one.
    ///
    /// # Emits Events
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::SecurityStructureAdded { id } }`
    async fn add_security_structure_of_factor_source_ids(
        &self,
        structure_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<()> {
        let id = structure_ids.metadata.id;
        let ids_of_factors_in_profile = self.factor_source_ids()?;
        let ids_in_structure = structure_ids
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
                    .append(structure_ids.clone())
                    .0)
            })
            .await?;

        if !inserted {
            return Err(CommonError::StructureAlreadyExists {
                bad_value: id.to_string(),
            });
        }

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::SecurityStructureAdded { id },
            ))
            .await;

        if !self.profile()?.has_any_main_security_structure() {
            self.set_main_security_structure(id).await?;
        }
        Ok(())
    }

    /// Updates an existing `SecurityStructureOfFactorSourceIDs` in the Profile.
    /// Returns an error if the structure does not exist or if it references unknown factors.
    /// Emits `Event::ProfileSaved` and `Event::ProfileModified::SecurityStructuresUpdated { ids }` on success.
    async fn update_security_structure_of_factor_source_ids(
        &self,
        structure_ids: &SecurityStructureOfFactorSourceIDs,
    ) -> Result<()> {
        let id = structure_ids.metadata.id;
        let ids_of_factors_in_profile = self.factor_source_ids()?;
        let ids_in_structure = structure_ids
            .all_factors()
            .into_iter()
            .cloned()
            .collect::<HashSet<FactorSourceID>>();

        let factors_only_in_structure =
            ids_in_structure.difference(&ids_of_factors_in_profile);
        let ids_of_missing_factors = factors_only_in_structure.collect_vec();

        if let Some(unknown_factor_source_id) = ids_of_missing_factors.first() {
            return Err(CommonError::StructureReferencesUnknownFactorSource {
                bad_value: unknown_factor_source_id.to_string(),
            });
        }

        self.update_profile_with(|p| {
            p.app_preferences
                .security
                .security_structures_of_factor_source_ids
                .try_update_with(&id, |s| {
                    *s = structure_ids.clone();
                })
                .map_err(|_| CommonError::UnknownSecurityStructureID {
                    id: id.to_string(),
                })?;
            Ok(())
        })
        .await?;

        Ok(())
    }

    /// Sets the Security Shield with the given `shield_id` as the main shield.
    /// If a main Security Shield already exists, it is removed and replaced with the new one.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::SecurityStructuresUpdated { id } }`
    async fn set_main_security_structure(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<()> {
        let updated_ids = self
            .update_profile_with(|p| p.set_main_security_structure(&shield_id))
            .await?;

        // Emit event
        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::SecurityStructuresUpdated {
                    ids: updated_ids,
                },
            ))
            .await;

        Ok(())
    }

    async fn entities_linked_to_security_structure(
        &self,
        shield_id: SecurityStructureID,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesLinkedToSecurityStructure> {
        let metadata = self
            .security_structure_of_factor_source_ids_by_security_structure_id(
                shield_id,
            )?
            .metadata;
        match profile_to_check {
            ProfileToCheck::Current => self
                .profile()?
                .current_network()?
                .entities_linked_to_security_structure(metadata),
            ProfileToCheck::Specific(specific_profile) => {
                let profile_network = specific_profile
                    .networks
                    .get_id(NetworkID::Mainnet)
                    .ok_or(CommonError::Unknown)?;
                profile_network.entities_linked_to_security_structure(metadata)
            }
        }
    }

    /// Returns all the Security Shields along with the number of entities linked to each Security Shield,
    /// either provisionally or currently securified.
    async fn get_shields_for_display(&self) -> Result<ShieldsForDisplay> {
        let security_structures =
            self.security_structures_of_factor_source_ids()?;

        let get_all_entities_linked_to_security_structures =
            security_structures.iter().map(|shield| {
                self.entities_linked_to_security_structure(
                    shield.metadata.id,
                    ProfileToCheck::Current,
                )
            });

        join_all(get_all_entities_linked_to_security_structures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()
            .map(|entities| {
                ShieldsForDisplay::from_iter(
                    entities.into_iter().map(ShieldForDisplay::with_linked),
                )
            })
    }

    /// Renames the Security Shield with the given `security_structure_id`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::SecurityStructuresUpdated { id } }`
    async fn rename_security_structure(
        &self,
        security_structure_id: SecurityStructureID,
        name: DisplayName,
    ) -> Result<()> {
        self.update_profile_with(|p| {
            p.set_security_structure_name(&security_structure_id, name)
        })
        .await
    }

    /// Returns the `SecurityStructuresOfFactorSources` based on the security state of the
    /// account or persona with given `address_of_account_or_persona`
    fn security_structure_of_factor_sources_from_address_of_account_or_persona(
        &self,
        address_of_account_or_persona: &AddressOfAccountOrPersona,
    ) -> Result<SecurityStructureOfFactorSources> {
        self.profile().and_then(|p| {
            let state = p
                .entity_by_address(*address_of_account_or_persona)?
                .security_state();
            let instances = state
                .as_securified()
                .ok_or(CommonError::SecurityStateNotSecurified)?
                .security_structure
                .clone();
            let ids = SecurityStructureOfFactorSourceIds::from(instances);
            TryFrom::<(&SecurityStructureOfFactorSourceIds, &FactorSources)>::try_from((&ids, &p.factor_sources))
        })
    }

    /// Returns the `SecurityStructuresOfFactorSources` based on the security state of the
    /// account or persona with given `address_of_account_or_persona`
    fn provisional_security_structure_of_factor_sources_from_address_of_account_or_persona(
        &self,
        address_of_account_or_persona: &AddressOfAccountOrPersona,
    ) -> Result<SecurityStructureOfFactorSources> {
        self.profile().and_then(|p| {
            let state = p
                .entity_by_address(*address_of_account_or_persona)?
                .security_state();
            let instances = state
                .get_provisional()
                .ok_or(CommonError::ProvisionalConfigInWrongStateExpectedInstancesDerived)?
                .get_security_structure_of_factor_instances()
                .clone();
            let ids = SecurityStructureOfFactorSourceIds::from(instances);
            TryFrom::<(&SecurityStructureOfFactorSourceIds, &FactorSources)>::try_from((&ids, &p.factor_sources))
        })
    }

    /// Returns the `FactorSource` set used to build the security structure
    /// in "decreasing friction" order
    fn sorted_factor_sources_from_security_structure(
        &self,
        structure: &SecurityStructureOfFactorSources,
    ) -> IndexSet<FactorSource> {
        let mut factors = structure
            .matrix_of_factors
            .all_factors()
            .into_iter()
            .cloned()
            .collect::<HashSet<FactorSource>>();
        _ = factors.insert(structure.authentication_signing_factor.clone());
        sort_group_factors(factors)
            .into_iter()
            .flat_map(|f| f.factor_sources())
            .collect()
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
    async fn test_unknown_shield_is_err() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        // ACT
        let result = os
            .security_structure_of_factor_source_ids_by_security_structure_id(
                SecurityStructureID::sample_other(),
            );
        // ASSERT
        assert!(matches!(
            result,
            Err(CommonError::UnknownSecurityStructureID { id: _ })
        ));
    }

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
        let structure_factor_id_level =
            SecurityStructureOfFactorSourceIDs::sample();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(
                &structure_factor_id_level,
            )
        })
        .await
        .unwrap();

        // ASSERT
        assert!(os
            .profile()
            .unwrap()
            .app_preferences
            .security
            .security_structures_of_factor_source_ids
            .contains_by_id(&structure_factor_id_level));
    }

    #[actix_rt::test]
    async fn when_adding_structure_referencing_unknown_factors_error_is_thrown()
    {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let structure_ids = SecurityStructureOfFactorSourceIDs::sample();
        let res = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_source_ids(&structure_ids)
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
        let structure_ids = SecurityStructureOfFactorSourceIDs::sample();
        let res = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_source_ids(&structure_ids)
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
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();

        // ACT
        let structure_ids = SecurityStructureOfFactorSourceIDs::sample();
        let id = structure_ids.metadata.id;
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(&structure_ids)
        })
        .await
        .unwrap();

        // ASSERT
        assert!(event_bus_driver.recorded().iter().any(|e| e.event
            == Event::ProfileModified {
                change: EventProfileModified::SecurityStructureAdded { id }
            }));
    }

    #[actix_rt::test]
    async fn add_first_structure_sets_it_as_main() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();

        // ACT
        let structure_ids = SecurityStructureOfFactorSourceIDs::sample_other();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(&structure_ids)
        })
        .await
        .unwrap();

        // ASSERT
        let added_structure = os
            .profile()
            .unwrap()
            .app_preferences
            .security
            .security_structures_of_factor_source_ids
            .iter()
            .find(|s| s.metadata.id == structure_ids.metadata.id)
            .unwrap();
        assert!(added_structure.metadata.is_main());
    }

    #[actix_rt::test]
    async fn when_setting_main_security_structure_with_invalid_id_error_is_thrown(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let structure_ids_sample_other =
            SecurityStructureOfFactorSourceIDs::sample_other();
        let result = os
            .set_main_security_structure(structure_ids_sample_other.id())
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::InvalidSecurityStructureID {
                bad_value: structure_ids_sample_other.id().to_string()
            })
        );
    }

    #[actix_rt::test]
    async fn given_existing_main_security_structure_when_updating_with_invalid_id_then_main_is_not_removed(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;
        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();

        let structure_factor_id_level =
            SecurityStructureOfFactorSourceIDs::sample();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(
                &structure_factor_id_level,
            )
        })
        .await
        .unwrap();

        // ACT
        let invalid_shield_id =
            SecurityStructureID::from(Uuid::from_bytes([0xab; 16]));
        let _ = os.set_main_security_structure(invalid_shield_id).await;

        // ASSERT
        let profile = os.profile().unwrap();
        let main_security_structure = profile
            .app_preferences
            .security
            .security_structures_of_factor_source_ids
            .first()
            .unwrap();

        assert!(main_security_structure.metadata.is_main());
    }

    #[actix_rt::test]
    async fn set_main_flag() {
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
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();

        let structure_ids_sample = SecurityStructureOfFactorSourceIDs::sample();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(&structure_ids_sample)
        })
        .await
        .unwrap();

        let structure_ids_sample_other =
            SecurityStructureOfFactorSourceIDs::sample_other();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(
                &structure_ids_sample_other,
            )
        })
        .await
        .unwrap();

        event_bus_driver.clear_recorded();

        // ACT
        assert!(structure_ids_sample.metadata.is_main());
        os.with_timeout(|x| {
            x.set_main_security_structure(
                structure_ids_sample_other.metadata.id(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let updated_security_structures = os
            .profile()
            .unwrap()
            .app_preferences
            .security
            .security_structures_of_factor_source_ids;

        let updated_structure_ids_sample = updated_security_structures.first();
        let updated_structure_ids_sample_other =
            updated_security_structures.get_at_index(1);

        assert!(!updated_structure_ids_sample.unwrap().metadata.is_main());
        assert!(updated_structure_ids_sample_other
            .unwrap()
            .metadata
            .is_main());

        let events = event_bus_driver.recorded();
        let security_structures_updated_event = events
            .iter()
            .find(|e| matches!(
                e.event,
                Event::ProfileModified {
                    change: EventProfileModified::SecurityStructuresUpdated { .. }
                }));
        let ids = if let Event::ProfileModified {
            change: EventProfileModified::SecurityStructuresUpdated { ref ids },
        } = &security_structures_updated_event.unwrap().event
        {
            ids
        } else {
            panic!("Expected a SecurityStructuresUpdated event");
        };

        assert_eq!(ids.len(), 2);
    }

    #[actix_rt::test]
    async fn set_main_flag_emits_event() {
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
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();

        let structure_ids_sample = SecurityStructureOfFactorSourceIDs::sample();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(&structure_ids_sample)
        })
        .await
        .unwrap();

        let structure_ids_sample_other =
            SecurityStructureOfFactorSourceIDs::sample_other();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(
                &structure_ids_sample_other,
            )
        })
        .await
        .unwrap();

        event_bus_driver.clear_recorded();

        // ACT
        os.with_timeout(|x| {
            x.set_main_security_structure(
                structure_ids_sample_other.metadata.id(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let events = event_bus_driver.recorded();
        assert!(events.iter().any(|e| e.event == Event::ProfileSaved),);
        assert!(events.iter().any(|e| e.event
            == Event::ProfileModified {
                change: EventProfileModified::SecurityStructuresUpdated {
                    ids: vec![
                        structure_ids_sample.metadata.id(),
                        structure_ids_sample_other.metadata.id()
                    ]
                }
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
        let structure_source_ids_sample =
            SecurityStructureOfFactorSourceIDs::sample();
        let structure_source_ids_sample_other =
            SecurityStructureOfFactorSourceIDs::sample_other();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(
                &structure_source_ids_sample,
            )
        })
        .await
        .unwrap();

        let result = os
            .with_timeout(|x| {
                x.add_security_structure_of_factor_source_ids(
                    &structure_source_ids_sample,
                )
            })
            .await;
        assert_eq!(
            result,
            Err(CommonError::StructureAlreadyExists {
                bad_value: structure_source_ids_sample.metadata.id.to_string()
            })
        );

        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(
                &structure_source_ids_sample_other,
            )
        })
        .await
        .unwrap();

        let structure_id_sample = SecurityStructureOfFactorSourceIDs::from(
            structure_source_ids_sample.clone(),
        );
        let structure_id_sample_other =
            SecurityStructureOfFactorSourceIDs::from(
                structure_source_ids_sample_other.clone(),
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

    #[actix_rt::test]
    async fn get_entities_only_provisionally_securified_to_secure_structure() {
        // ARRANGE
        let os = SargonOS::fast_boot().await;
        let shield_id = add_unsafe_shield(&os).await.unwrap();

        // Two visible accounts
        let accounts = Accounts::from_iter([
            Account::sample_mainnet_alice(),
            Account::sample_mainnet_bob(),
        ]);
        // One hidden account
        let hidden_accounts =
            Accounts::from_iter([Account::sample_mainnet_diana()]);
        // Two visible personas
        let personas = Personas::from_iter([
            Persona::sample_mainnet_batman(),
            Persona::sample_mainnet_satoshi(),
        ]);
        // One hidden persona
        let hidden_personas =
            Personas::from_iter([Persona::sample_mainnet_turing()]);

        let all_accounts = accounts
            .iter()
            .chain(hidden_accounts.iter())
            .collect::<Accounts>();
        let all_personas = personas
            .iter()
            .chain(hidden_personas.iter())
            .collect::<Personas>();

        os.add_accounts(all_accounts.clone()).await.unwrap();
        os.add_personas(all_personas.clone()).await.unwrap();

        let addresses: IndexSet<AddressOfAccountOrPersona> = all_accounts
            .iter()
            .map(|account| account.address().into())
            .chain(all_personas.iter().map(|persona| persona.address().into()))
            .collect();

        // ACT
        os.apply_security_shield_with_id_to_entities(shield_id, addresses)
            .await
            .unwrap();

        let result = os
            .entities_linked_to_security_structure(
                shield_id,
                ProfileToCheck::Current,
            )
            .await
            .unwrap();

        // ASSERT
        let updated_accounts = os.accounts_on_current_network().unwrap();
        assert_eq!(result.accounts, updated_accounts);
        let updated_hidden_accounts = os
            .profile()
            .unwrap()
            .hidden_accounts_on_current_network()
            .unwrap();
        assert_eq!(result.hidden_accounts, updated_hidden_accounts);
        let updated_personas = os.personas_on_current_network().unwrap();
        assert_eq!(result.personas, updated_personas);
        let updated_hidden_personas = os
            .profile()
            .unwrap()
            .hidden_personas_on_current_network()
            .unwrap();
        assert_eq!(result.hidden_personas, updated_hidden_personas)
    }

    #[actix_rt::test]
    async fn get_entities_currently_and_provisionally_securified_to_secure_structure(
    ) {
        // ARRANGE
        let os = SargonOS::fast_boot().await;
        let shield = add_unsafe_shield_with_matrix(&os).await.unwrap();
        let shield_id = shield.id();

        let accounts = Accounts::from_iter([
            Account::sample_mainnet_alice(),
            Account::sample_mainnet_bob(),
        ]);
        let personas = Personas::from_iter([
            Persona::sample_mainnet_satoshi(),
            Persona::sample_mainnet_batman(),
        ]);

        let all_accounts = accounts.iter().collect::<Accounts>();
        let all_personas = personas.iter().collect::<Personas>();

        os.add_accounts(all_accounts.clone()).await.unwrap();
        os.add_personas(all_personas.clone()).await.unwrap();

        let addresses: IndexSet<AddressOfAccountOrPersona> = all_accounts
            .iter()
            .map(|account| account.address().into())
            .chain(all_personas.iter().map(|persona| persona.address().into()))
            .collect();

        os.apply_security_shield_with_id_to_entities(shield_id, addresses)
            .await
            .unwrap();

        // ACT
        let mut account_alice = os
            .account_by_address(accounts.first().unwrap().address())
            .unwrap();

        let mut account_security_structure_of_instances = account_alice
            .get_provisional()
            .unwrap()
            .as_factor_instances_derived()
            .unwrap()
            .clone();
        account_security_structure_of_instances
            .authentication_signing_factor_instance =
            HierarchicalDeterministicFactorInstance::sample_other();
        let account_secured_control = SecuredEntityControl::new(
            account_alice
                .clone()
                .security_state()
                .as_unsecured()
                .unwrap()
                .transaction_signing
                .clone(),
            AccessControllerAddress::sample_mainnet(),
            account_security_structure_of_instances,
        )
        .unwrap();
        account_alice
            .set_security_state(EntitySecurityState::Securified {
                value: account_secured_control,
            })
            .unwrap();
        os.update_account(account_alice.clone()).await.unwrap();

        let mut persona_satoshi = os
            .persona_by_address(personas.first().unwrap().address())
            .unwrap();

        let persona_security_structure_of_instances = persona_satoshi
            .get_provisional()
            .unwrap()
            .as_factor_instances_derived()
            .unwrap()
            .clone();
        let persona_secured_control = SecuredEntityControl::new(
            persona_satoshi
                .clone()
                .security_state()
                .as_unsecured()
                .unwrap()
                .transaction_signing
                .clone(),
            AccessControllerAddress::sample_mainnet_other(),
            persona_security_structure_of_instances,
        )
        .unwrap();
        persona_satoshi
            .set_security_state(EntitySecurityState::Securified {
                value: persona_secured_control,
            })
            .unwrap();
        os.update_persona(persona_satoshi.clone()).await.unwrap();

        let result = os
            .entities_linked_to_security_structure(
                shield_id,
                ProfileToCheck::Current,
            )
            .await
            .unwrap();

        // ASSERT
        let updated_accounts = os.accounts_on_current_network().unwrap();
        assert_eq!(result.accounts, updated_accounts);

        let is_account_alice_currently_securified = updated_accounts
            .first()
            .unwrap()
            .security_state()
            .is_currently_securified_with(shield_id);
        assert!(is_account_alice_currently_securified);
        let is_account_bob_provisionally_securified = updated_accounts
            .get_at_index(1)
            .unwrap()
            .security_state()
            .is_provisionally_securified_with(shield_id);
        assert!(is_account_bob_provisionally_securified);

        let updated_personas = os.personas_on_current_network().unwrap();
        assert_eq!(result.personas, updated_personas);

        let is_persona_satoshi_currently_securified = updated_personas
            .first()
            .unwrap()
            .security_state()
            .is_currently_securified_with(shield_id);
        assert!(is_persona_satoshi_currently_securified);
        let is_persona_batman_provisionally_securified = updated_personas
            .get_at_index(1)
            .unwrap()
            .security_state()
            .is_provisionally_securified_with(shield_id);
        assert!(is_persona_batman_provisionally_securified);
    }

    #[actix_rt::test]
    async fn test_get_entities_linked_to_secure_structure_for_specific_profile()
    {
        // ARRANGE
        // Verify the entities when checking for a specific Profile
        // (which will check on Mainnet, regardless of the current network set on Profile)
        let mut profile_to_check = Profile::sample();
        profile_to_check
            .app_preferences
            .gateways
            .change_current(Gateway::stokenet());
        let structure_ids_sample_other =
            SecurityStructureOfFactorSourceIDs::sample();
        profile_to_check
            .app_preferences
            .security
            .security_structures_of_factor_source_ids
            .append(structure_ids_sample_other.clone());

        let (os, shield_id, account, persona) = {
            let os = SargonOS::fast_boot().await;
            let shield_id = add_unsafe_shield(&os).await.unwrap();
            let network = NetworkID::Mainnet;
            let account = os
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();
            let persona = os
                .create_and_save_new_persona_with_bdfs(
                    network,
                    DisplayName::sample_other(),
                    None,
                )
                .await
                .unwrap();
            (os, shield_id, account, persona)
        };

        // add security shield on mainnet entities
        os.apply_security_shield_with_id_to_entities(
            shield_id,
            [
                AddressOfAccountOrPersona::from(account.address()),
                AddressOfAccountOrPersona::from(persona.address()),
            ]
            .iter()
            .cloned()
            .collect(),
        )
        .await
        .unwrap();

        // ACT
        let result = os
            .entities_linked_to_security_structure(
                shield_id,
                ProfileToCheck::Specific(profile_to_check),
            )
            .await
            .unwrap();

        // ASSERT
        assert!(result.accounts.is_empty());
        assert!(result.hidden_accounts.is_empty());
        assert!(result.personas.is_empty());
        assert!(result.hidden_personas.is_empty())
    }

    #[actix_rt::test]
    async fn test_specific_profile_mainnet_missing() {
        // Test the failure case when checking entities for a specific Profile that doesn't have Mainnet in its networks
        let profile = Profile::sample_other();
        let os = SargonOS::fast_boot().await;
        let shield_id = add_unsafe_shield(&os).await.unwrap();

        let result = os
            .entities_linked_to_security_structure(
                shield_id,
                ProfileToCheck::Specific(profile),
            )
            .await
            .expect_err("Expected an error");
        assert_eq!(result, CommonError::Unknown);
    }

    #[actix_rt::test]
    async fn test_get_shields_for_display() {
        // ARRANGE
        let os = SargonOS::fast_boot().await;
        let shield_id_sample = add_unsafe_shield(&os).await.unwrap();
        let shield_id_sample_other =
            add_unsafe_shield_with_matrix_with_fixed_metadata(
                &os,
                SecurityStructureMetadata::sample_other(),
            )
            .await
            .unwrap()
            .id();

        let main_account = Account::sample_mainnet_carol();

        // some other accounts and personas
        let accounts = Accounts::from_iter([
            Account::sample_mainnet_alice(),
            Account::sample_mainnet_bob(),
        ]);
        let hidden_accounts =
            Accounts::from_iter([Account::sample_mainnet_diana()]);
        let personas = Personas::from_iter([
            Persona::sample_mainnet_batman(),
            Persona::sample_mainnet_satoshi(),
        ]);
        let hidden_personas =
            Personas::from_iter([Persona::sample_mainnet_turing()]);

        let all_accounts = accounts
            .iter()
            .chain(hidden_accounts.iter())
            .chain(__std_iter::once(main_account.clone()))
            .collect::<Accounts>();
        let all_personas = personas
            .iter()
            .chain(hidden_personas.iter())
            .collect::<Personas>();

        os.add_accounts(all_accounts.clone()).await.unwrap();
        os.add_personas(all_personas.clone()).await.unwrap();

        let address_of_main_account: IndexSet<AddressOfAccountOrPersona> =
            [main_account.address().into()].into_iter().collect();

        let addresses_of_rest_entities: IndexSet<AddressOfAccountOrPersona> =
            all_accounts
                .iter()
                .map(|account| account.address().into())
                .chain(
                    all_personas.iter().map(|persona| persona.address().into()),
                )
                .collect();

        // ACT
        os.apply_security_shield_with_id_to_entities(
            shield_id_sample,
            addresses_of_rest_entities,
        )
        .await
        .unwrap();

        os.apply_security_shield_with_id_to_entities(
            shield_id_sample_other,
            address_of_main_account,
        )
        .await
        .unwrap();

        let result = os.get_shields_for_display().await.unwrap();

        // ASSERT
        assert_eq!(result.iter().count(), 2);
        assert_eq!(result.first().unwrap().metadata.id, shield_id_sample);
        assert_eq!(result.first().unwrap().number_of_linked_accounts, 2);
        assert_eq!(result.first().unwrap().number_of_linked_hidden_accounts, 1);
        assert_eq!(result.first().unwrap().number_of_linked_personas, 2);
        assert_eq!(result.first().unwrap().number_of_linked_hidden_personas, 1);
        assert_eq!(
            result.get_at_index(1).unwrap().metadata.id,
            shield_id_sample_other
        );
        assert_eq!(
            result.get_at_index(1).unwrap().number_of_linked_accounts,
            1
        );
        assert_eq!(
            result
                .get_at_index(1)
                .unwrap()
                .number_of_linked_hidden_accounts,
            0
        );
        assert_eq!(
            result.get_at_index(1).unwrap().number_of_linked_personas,
            0
        );
        assert_eq!(
            result
                .get_at_index(1)
                .unwrap()
                .number_of_linked_hidden_personas,
            0
        )
    }

    #[actix_rt::test]
    async fn rename_structure_success() {
        // ARRANGE
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let mut clients = Clients::new(Bios::new(drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os = actix_rt::time::timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SargonOS::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();
        os.with_timeout(|x| x.new_wallet()).await.unwrap();
        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();

        let structure = SecurityStructureOfFactorSourceIDs::sample();
        let id = structure.metadata.id;
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(&structure)
        })
        .await
        .unwrap();

        // ACT
        let new_name = DisplayName::new("Renamed Shield").unwrap();
        os.with_timeout(|x| x.rename_security_structure(id, new_name))
            .await
            .unwrap();

        // ASSERT
        let profile = os.profile().unwrap();
        let updated = profile
            .app_preferences
            .security
            .security_structures_of_factor_source_ids
            .get_id(id)
            .unwrap();
        assert_eq!(updated.metadata.display_name, new_name);
    }

    #[actix_rt::test]
    async fn rename_structure_unknown_id_returns_error() {
        // ARRANGE
        let os = SargonOS::fast_boot().await;
        let unknown_id = SecurityStructureID::sample_other();
        let new_name = DisplayName::new("Should Fail").unwrap();

        // ACT
        let result = os.rename_security_structure(unknown_id, new_name).await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::InvalidSecurityStructureID {
                bad_value: unknown_id.to_string()
            })
        );
    }

    #[actix_rt::test]
    async fn update_structure_success() {
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
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();

        // ACT
        let structure_ids = SecurityStructureOfFactorSourceIDs::sample();
        let id = structure_ids.metadata.id;
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(&structure_ids)
        })
        .await
        .unwrap();
        let mut updated_structure_ids =
            SecurityStructureOfFactorSourceIDs::sample_other();
        updated_structure_ids.metadata.id = id;
        os.with_timeout(|x| {
            x.update_security_structure_of_factor_source_ids(
                &updated_structure_ids,
            )
        })
        .await
        .unwrap();

        // ASSERT
        let profile = os.profile().unwrap();
        let updated = profile
            .app_preferences
            .security
            .security_structures_of_factor_source_ids
            .get_id(id)
            .unwrap();
        assert_eq!(updated.clone(), updated_structure_ids);
    }

    #[actix_rt::test]
    async fn update_structure_unknown_id_returns_error() {
        // ARRANGE
        let os = SargonOS::fast_boot().await;
        os.debug_add_all_sample_hd_factor_sources().await.unwrap();

        let structure = SecurityStructureOfFactorSourceIDs::sample();

        // ACT
        let result = os
            .update_security_structure_of_factor_source_ids(&structure)
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::UnknownSecurityStructureID {
                id: structure.metadata.id.to_string()
            })
        );
    }

    #[actix_rt::test]
    async fn update_structure_referencing_unknown_factors_returns_error() {
        // ARRANGE: No factor sources added to the profile
        let os = SargonOS::fast_boot().await;

        // This structure references factor IDs that the profile doesn't know about
        let structure_ids = SecurityStructureOfFactorSourceIDs::sample();

        // ACT
        let res = os
            .with_timeout(|x| {
                x.update_security_structure_of_factor_source_ids(&structure_ids)
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

    // rust
    #[actix_rt::test]
    async fn security_structure_of_factor_sources_from_address_of_account_or_persona_when_securified(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;
        os.add_factor_source(FactorSource::sample_device())
            .await
            .unwrap();

        let account = Account::sample_securified_mainnet(
            "Carla",
            2,
            HierarchicalDeterministicFactorInstance::sample_fi0(
                CAP26EntityKind::Account,
            ),
            || {
                let idx =
                    Hardened::from_local_key_space(2u32, IsSecurified(true))
                        .unwrap();
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r2(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        idx,
                    )
                )
            },
        );
        os.add_account(account.clone()).await.unwrap();

        // ACT
        let address: AddressOfAccountOrPersona = account.address().into();
        let result = os
            .security_structure_of_factor_sources_from_address_of_account_or_persona(&address);

        // ASSERT
        let _ = result.unwrap();
    }

    #[actix_rt::test]
    async fn security_structure_of_factor_sources_from_address_of_account_or_persona_when_not_securified_errors(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // Create and persist an account but do NOT securify it
        let account = os
            .create_and_save_new_account_with_bdfs(
                NetworkID::Mainnet,
                DisplayName::sample(),
            )
            .await
            .unwrap();

        let address: AddressOfAccountOrPersona = account.address().into();

        // ACT
        let result = os
            .security_structure_of_factor_sources_from_address_of_account_or_persona(&address);

        // ASSERT
        assert_eq!(result, Err(CommonError::SecurityStateNotSecurified));
    }

    #[actix_rt::test]
    async fn sorted_factor_sources_from_security_structure_sorts_dedups_and_includes_auth(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // Ensure all sample factor sources are present in the profile so the structure can be resolved at source level.
        for fs in FactorSources::sample_values_all_hd().into_iter() {
            os.add_factor_source(fs).await.unwrap();
        }

        // Create and persist a sample structure (ID level), then get its source-level counterpart.
        let structure_ids = SecurityStructureOfFactorSourceIDs::sample();
        os.with_timeout(|x| {
            x.add_security_structure_of_factor_source_ids(&structure_ids)
        })
        .await
        .unwrap();

        let structure_id =
            SecurityStructureOfFactorSourceIDs::from(structure_ids.clone());
        let structure = os
            .security_structure_of_factor_sources_from_security_structure_of_factor_source_ids(
                &structure_id,
            )
            .unwrap();

        // ACT
        let sorted = os
            .sorted_factor_sources_from_security_structure(&structure.clone());

        // Build the expected set: union of all matrix factors + the authentication signing factor,
        // then sorted using the same grouping/sorting logic the function promises.
        let mut union = structure
            .matrix_of_factors
            .all_factors()
            .into_iter()
            .cloned()
            .collect::<HashSet<FactorSource>>();
        // Ensure the auth-signing factor is included even if not present in the matrix.
        _ = union.insert(structure.authentication_signing_factor);

        let expected: IndexSet<FactorSource> = sort_group_factors(union)
            .into_iter()
            .flat_map(|g| g.factor_sources())
            .collect();

        // ASSERT
        // Exact sequence and contents should match the sorted expectation.
        assert_eq!(sorted, expected);
    }
}

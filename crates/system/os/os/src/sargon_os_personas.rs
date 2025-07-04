use crate::prelude::*;

// ==================
// Create Unsaved Persona(s)
// ==================
impl SargonOS {
    /// Returns the non-hidden personas on the current network, empty if no personas
    /// on the network
    pub fn personas_on_current_network(&self) -> Result<Personas> {
        self.profile_state_holder.personas_on_current_network()
    }

    /// Looks up the persona by persona address, returns Err if the persona is
    /// unknown, will return a hidden persona if queried for.
    pub fn persona_by_address(
        &self,
        address: IdentityAddress,
    ) -> Result<Persona> {
        self.profile_state_holder.persona_by_address(address)
    }

    /// Create a new Persona with main BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_persona_with_main_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
        persona_data: Option<PersonaData>,
    ) -> Result<Persona> {
        let bdfs = self.main_bdfs()?;
        self.create_and_save_new_persona_with_factor_source(
            bdfs.into(),
            network_id,
            name,
            persona_data,
        )
        .await
    }

    /// Create a new Persona and adds it to the active Profile.
    ///
    /// # Emits Events
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        persona_data: Option<PersonaData>,
    ) -> Result<Persona> {
        let profile = self.profile()?;
        let key_derivation_interactor = self.keys_derivation_interactor();
        let mut persona: Persona = profile
            .create_unsaved_persona_with_factor_source(
                factor_source.clone(),
                network_id,
                name,
                key_derivation_interactor,
            )
            .await?;

        if let Some(persona_data) = persona_data {
            persona.persona_data = persona_data;
        }

        self.update_last_used_of_factor_source(factor_source.id())
            .await?;
        self.add_persona(persona.clone()).await?;
        Ok(persona)
    }

    pub async fn create_and_save_new_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        persona_data: Option<PersonaData>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        self.spot_check_factor_source_before_entity_creation_if_necessary(
            factor_source.clone(),
            network_id,
            EntityKind::Persona,
        )
        .await?;
        debug!("Creating persona.");
        let (mut persona, instances_in_cache_consumer, derivation_outcome) = self
            .create_unsaved_persona_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                name,
            )
            .await?;
        // If PersonaData is set, assign it before saving it into Profile
        if let Some(persona_data) = persona_data {
            persona.persona_data = persona_data;
        }
        debug!("Created persona, requesting authorization...");

        let authorization = self
            .authorization_interactor()
            .request_authorization(AuthorizationPurpose::CreatingPersona)
            .await;

        match authorization {
            AuthorizationResponse::Rejected => {
                debug!("User rejected authorization, aborting.");
                return Err(CommonError::HostInteractionAborted);
            }
            AuthorizationResponse::Authorized => {
                debug!("User authorized, saving to profile...");
            }
        }

        // First try save Persona into Profile...
        self.add_persona(persona.clone()).await?;
        // ... if success, then delete FactorInstance from cache!
        instances_in_cache_consumer.consume().await?;

        info!(
            "Created persona and saved new persona into profile, address: {}",
            persona.address
        );
        Ok((persona, derivation_outcome))
    }

    pub async fn create_unsaved_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<(
        Persona,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let key_derivation_interactors = self.keys_derivation_interactor();

        let profile = self.profile()?;

        let future = profile
            .create_unsaved_persona_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                name,
                Arc::new(self.clients.factor_instances_cache.clone()),
                key_derivation_interactors,
            );

        let (
            factor_source_id,
            persona,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = future.await?;

        // TODO: move this to the FactorInstancesProvider... it should take a `emit_last_used` closure
        // Change of `last_used_on` of FactorSource
        self.update_last_used_of_factor_source(factor_source_id)
            .await?;

        Ok((persona, instances_in_cache_consumer, derivation_outcome))
    }
}

// ==================
// Add (Save) Persona(s)
// ==================
impl SargonOS {
    /// Add the `persona` to active profile and **saves** the updated profile to
    /// secure storage.
    ///
    /// Returns `Ok(())` if the `persona` was new and successfully added. If
    /// saving failed or if the persona was already present in Profile, an
    /// error is returned.
    ///
    /// # Emits Events
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::PersonasAdded { addresses } }`
    pub async fn add_persona(&self, persona: Persona) -> Result<()> {
        self.add_entity(persona).await
    }

    /// Adds the `personas` to active profile and **saves** the updated profile to
    /// secure storage.
    ///
    /// Returns `Ok(())` if the `personas` were new and successfully added. If
    /// saving failed or if the personas were already present in Profile, an
    /// error is returned.
    ///
    /// # Emits Events
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::PersonasAdded { addresses } }`
    pub async fn add_personas(&self, personas: Personas) -> Result<()> {
        self.add_entities(personas).await.map_err(|e| match e {
            CommonError::UnableToAddAllEntitiesDuplicatesFound => {
                CommonError::UnableToAddAllPersonasDuplicatesFound
            }
            _ => e,
        })
    }
}

// ==================
// Update Persona(s)
// ==================
impl SargonOS {
    /// Updates the persona `updated` by mutating current profile and persisting
    /// the change to secure storage. Throws `UnknownPersona` error if the persona
    /// is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaUpdated { address } }`
    pub async fn update_persona(&self, updated: Persona) -> Result<()> {
        self.update_entity(updated).await
    }

    /// Updates the personas `updated` by mutating current profile and persisting
    /// the change to secure storage. Throws `UnknownPersona` error if any of the persona
    /// is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonasUpdated { addresses } }`
    pub async fn update_personas(&self, updated: Personas) -> Result<()> {
        self.update_entities(updated).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_first_add_persona() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.add_persona(Persona::sample()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.profile().unwrap().networks[0].personas.len(), 1);
    }

    #[actix_rt::test]
    async fn test_content_hint_is_updated_when_personas_are_added() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.add_persona(Persona::sample()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile()
                .unwrap()
                .header
                .content_hint
                .number_of_personas_on_all_networks_in_total,
            1
        );
        assert_eq!(
            os.profile().unwrap().header.content_hint.number_of_networks,
            1
        );
    }

    #[actix_rt::test]
    async fn test_first_create_unsaved_persona() {
        // ARRANGE
        let os = SUT::fast_boot_bdfs(MnemonicWithPassphrase::sample()).await;

        // ACT
        let (mut unsaved_persona, _) = os
            .with_timeout(|x| {
                x.create_unsaved_mainnet_persona_with_main_bdfs(
                    DisplayName::new("Satoshi").unwrap(),
                )
            })
            .await
            .unwrap();

        // ASSERT
        unsaved_persona.persona_data = Persona::sample().persona_data;
        pretty_assertions::assert_eq!(unsaved_persona, Persona::sample());
        assert_eq!(os.profile().unwrap().networks[0].personas.len(), 0); // not added
    }

    #[actix_rt::test]
    async fn test_create_unsaved_persona_twice_yield_same_personas_if_instances_in_cache_consumer_is_not_used(
    ) {
        // ARRANGE
        let (os, bdfs) = SUT::with_bdfs().await;

        // ACT
        let (first, instances_in_cache_consumer) = os
            .with_timeout(|x| {
                x.create_unsaved_persona_with_factor_source(
                    bdfs.clone(),
                    NetworkID::Mainnet,
                    DisplayName::new("Unnamed").unwrap(),
                )
            })
            .await
            .unwrap();

        // Not used!
        drop(instances_in_cache_consumer);

        let (second, _) = os
            .with_timeout(|x| {
                x.create_unsaved_persona_with_main_bdfs(
                    NetworkID::Mainnet,
                    DisplayName::new("Unnamed").unwrap(),
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(first, second);
    }

    #[actix_rt::test]
    async fn test_create_unsaved_persona_twice_different_persona_if_instances_are_consumed(
    ) {
        // ARRANGE
        let os = SUT::fast_boot_bdfs(MnemonicWithPassphrase::sample()).await;

        // ACT
        let (first, instances_in_cache_consumer) = os
            .with_timeout(|x| {
                x.create_unsaved_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // Consume!
        instances_in_cache_consumer.consume().await.unwrap();

        let (second, _) = os
            .with_timeout(|x| {
                x.create_unsaved_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_ne!(first, second);
    }

    #[actix_rt::test]
    async fn test_first_create_and_add_persona_is_added() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let persona = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile().unwrap().networks[0].personas,
            Personas::just(persona)
        );
    }

    #[actix_rt::test]
    async fn test_create_and_save_new_persona_sets_persona_data() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let persona = os
            .with_timeout(|x| {
                x.create_and_save_new_persona_with_main_bdfs(
                    NetworkID::Mainnet,
                    DisplayName::sample(),
                    Some(PersonaData::sample()),
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile().unwrap().networks[0].personas,
            Personas::just(persona)
        );
        assert_eq!(
            os.profile().unwrap().networks[0].personas[0].persona_data,
            PersonaData::sample()
        );
    }

    #[actix_rt::test]
    async fn test_first_create_and_add_persona_has_index_0() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let persona = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            persona
                .security_state
                .clone()
                .into_unsecured()
                .unwrap()
                .transaction_signing
                .derivation_path()
                .index(),
            HDPathComponent::unsecurified_hardened(0).unwrap()
        );
    }

    #[actix_rt::test]
    async fn test_second_create_and_add_persona_has_index_1() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let _ = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        let second = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            second
                .security_state
                .clone()
                .into_unsecured()
                .unwrap()
                .transaction_signing
                .derivation_path()
                .index(),
            HDPathComponent::unsecurified_hardened(1).unwrap()
        );
    }

    #[actix_rt::test]
    async fn batch_create_persona_then_n_personas_are_saved_and_have_indices_0_through_n(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n: u32 = 10;
        for _ in 0..n {
            os.with_timeout(|os| {
                os.create_and_save_new_mainnet_persona_with_main_bdfs(
                    DisplayName::sample(),
                )
            })
            .await
            .unwrap();
        }

        // ASSERT
        let indices = os.profile().unwrap().networks[0]
            .personas
            .iter()
            .map(|x| {
                x.security_state
                    .clone()
                    .into_unsecured()
                    .unwrap()
                    .transaction_signing
                    .derivation_path()
                    .index()
            })
            .map(|i| u32::from(i.index_in_local_key_space()))
            .collect_vec();
        assert_eq!(indices, (0u32..n).collect_vec());

        os.profile().unwrap().networks[0]
            .personas
            .iter()
            .for_each(|p| assert_eq!(p.persona_data, PersonaData::default()))
    }

    #[actix_rt::test]
    async fn update_persona_updates_in_memory_profile() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let mut persona = Persona::sample();
        os.with_timeout(|x| x.add_persona(persona.clone()))
            .await
            .unwrap();

        // ACT
        persona.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_persona(persona.clone()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile().unwrap().networks[0].personas[0],
            persona.clone()
        )
    }

    #[actix_rt::test]
    async fn update_persona_updates_saved_profile() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let mut persona = Persona::sample();
        os.with_timeout(|x| x.add_persona(persona.clone()))
            .await
            .unwrap();

        // ACT
        persona.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_persona(persona.clone()))
            .await
            .unwrap();

        // ASSERT
        let saved_profile = os
            .with_timeout(|x| x.secure_storage.load_profile())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(saved_profile.networks[0].personas[0], persona.clone())
    }

    #[actix_rt::test]
    async fn test_update_persona_emits() {
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
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        let mut persona = Persona::sample();
        os.with_timeout(|x| x.add_persona(persona.clone()))
            .await
            .unwrap();

        // ACT
        persona.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_persona(persona.clone()))
            .await
            .unwrap();

        // ASSERT
        assert!(event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::PersonaUpdated));
    }

    #[actix_rt::test]
    async fn test_update_personas_emits() {
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
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        let mut persona = Persona::sample();
        let mut persona2 = Persona::sample_other();
        os.with_timeout(|x| {
            x.add_personas(Personas::from_iter([
                persona.clone(),
                persona2.clone(),
            ]))
        })
        .await
        .unwrap();

        // ACT
        persona.display_name = DisplayName::random();
        persona2.display_name = DisplayName::random();
        os.with_timeout(|x| {
            x.update_personas(Personas::from_iter([
                persona.clone(),
                persona2.clone(),
            ]))
        })
        .await
        .unwrap();

        // ASSERT
        assert!(event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::PersonasUpdated));
    }

    #[actix_rt::test]
    async fn update_persona_unknown_personas_throws() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let result = os
            .with_timeout(|x| x.update_persona(Persona::sample()))
            .await;

        // ASSERT
        assert_eq!(result, Err(CommonError::UnknownPersona))
    }

    #[actix_rt::test]
    async fn add_personas_empty_is_ok() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let result = os.with_timeout(|x| x.add_personas(Personas::new())).await;

        // ASSERT
        assert!(result.is_ok())
    }

    #[actix_rt::test]
    async fn add_two_personas() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| {
            x.add_personas(Personas::from_iter([
                Persona::sample(),
                Persona::sample_other(),
            ]))
        })
        .await
        .unwrap();

        // ASSERT
        assert_eq!(os.profile().unwrap().networks[0].personas.len(), 2)
    }

    #[actix_rt::test]
    async fn add_personas_duplicates_throws() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let persona = Persona::sample();
        os.with_timeout(|x| x.add_persona(persona.clone()))
            .await
            .unwrap();

        // ACT
        let result = os
            .with_timeout(|x| x.add_personas(Personas::just(persona.clone())))
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::UnableToAddAllPersonasDuplicatesFound)
        )
    }

    #[actix_rt::test]
    async fn test_personas_on_current_network_empty() {
        let os = SUT::fast_boot().await;
        assert_eq!(os.personas_on_current_network().unwrap(), Personas::new());
    }

    #[actix_rt::test]
    async fn test_personas_on_current_network_non_empty() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let persona = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.personas_on_current_network().unwrap(),
            Personas::just(persona)
        );
    }

    #[actix_rt::test]
    async fn test_personas_on_current_network_empty_when_switched_network() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let _ = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // ACT
        let _ = os
            .with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.personas_on_current_network().unwrap(), Personas::new());
    }

    #[actix_rt::test]
    async fn test_persona_by_address_exists() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let persona = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.persona_by_address(persona.address), Ok(persona));
    }

    #[actix_rt::test]
    async fn test_persona_by_address_not_exists() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        // so that we have at least one network (with one persona)
        let _ = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_main_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.persona_by_address(IdentityAddress::sample_mainnet()),
            Err(CommonError::UnknownPersona)
        );
    }
}

use std::{borrow::Borrow, sync::RwLockWriteGuard};

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

    /// Creates a new unsaved mainnet persona named "Unnamed {N}", where `N` is the
    /// index of the next persona for the BDFS.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_persona_with_bdfs(
        &self,
    ) -> Result<(Persona, InstancesConsumer)> {
        let bdfs = self.bdfs()?;
        self.create_unsaved_unnamed_mainnet_persona_with_factor_source(
            bdfs.into(),
        )
        .await
    }

    /// Creates a new unsaved mainnet persona named "Unnamed {N}", where `N` is the
    /// index of the next persona for the selected factor_source.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<(Persona, InstancesConsumer)> {
        self.create_unsaved_persona_with_factor_source(
            factor_source,
            NetworkID::Mainnet,
            DisplayName::new("Unnamed").unwrap(),
        )
        .await
    }

    /// Uses `create_unsaved_persona` specifying `NetworkID::Mainnet` using
    /// the specified `factor_source`.
    pub async fn create_unsaved_mainnet_persona_with_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<(Persona, InstancesConsumer)> {
        let bdfs = self.bdfs()?;
        self.create_unsaved_mainnet_persona_with_factor_source(
            bdfs.into(),
            name,
        )
        .await
    }

    /// Uses `create_unsaved_persona` specifying `NetworkID::Mainnet` using
    /// the specified `factor_source`.
    pub async fn create_unsaved_mainnet_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<(Persona, InstancesConsumer)> {
        self.create_unsaved_persona_with_factor_source(
            factor_source,
            NetworkID::Mainnet,
            name,
        )
        .await
    }

    /// Creates a new non securified persona **WITHOUT** adding it to Profile,
    /// using the *main* "Babylon" `DeviceFactorSource` and the "next" index for
    /// this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `os.add_persona(persona)`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage, since the `last_used_on` date
    /// of the factor source has been updated.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn create_unsaved_persona_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<(Persona, InstancesConsumer)> {
        let bdfs = self.bdfs()?;
        self.create_unsaved_persona_with_factor_source(
            bdfs.into(),
            network_id,
            name,
        )
        .await
    }

    /// Creates a new non securified persona **WITHOUT** adding it to Profile,
    /// using specified factor source and the "next" index for this FactorSource
    ///
    /// If you want to add it to Profile, call `os.add_persona(persona)`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage, since the `last_used_on` date
    /// of the factor source has been updated.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn create_unsaved_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<(Persona, InstancesConsumer)> {
        self.create_unsaved_persona_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            name,
        )
        .await
        .map(|(x, y, _)| (x, y))
    }

    pub async fn create_unsaved_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<(
        Persona,
        InstancesConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let key_derivation_interactors = self.keys_derivation_interactors();

        let profile = self.profile()?;

        let (factor_source_id, persona, instances_consumer, derivation_outcome) = profile
            .create_unsaved_persona_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                name,
                Arc::new(self.clients.factor_instances_cache.clone()),
                key_derivation_interactors,
            )
            .await?;

        // TODO: move this to the FactorInstancesProvider... it should take a `emit_last_used` closure
        // Change of `last_used_on` of FactorSource
        self.update_last_used_of_factor_source(factor_source_id)
            .await?;

        Ok((persona, instances_consumer, derivation_outcome))
    }

    /// Create a new mainnet Persona named "Unnamed" using BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_unnamed_mainnet_persona_with_bdfs(
        &self,
    ) -> Result<Persona> {
        let bdfs = self.bdfs()?;
        self.create_and_save_new_unnamed_mainnet_persona_with_factor_source(
            bdfs.into(),
        )
        .await
    }

    /// Create a new mainnet Persona named "Unnamed" using selected factor source and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_unnamed_mainnet_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<Persona> {
        self.create_and_save_new_mainnet_persona_with_factor_source(
            factor_source,
            DisplayName::new("Unnamed").unwrap(),
        )
        .await
    }

    /// Create a new mainnet Persona using the BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_mainnet_persona_with_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<Persona> {
        self.create_and_save_new_mainnet_persona_with_bdfs_with_derivation_outcome(name).await.map(|(x, _)| x)
    }

    pub async fn create_and_save_new_mainnet_persona_with_bdfs_with_derivation_outcome(
        &self,
        name: DisplayName,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let bdfs = self.bdfs()?;
        self.create_and_save_new_mainnet_persona_with_factor_source_with_derivation_outcome(
            bdfs.into(),
            name,
        )
        .await
    }

    /// Create a new mainnet Persona using the selected factor source and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_mainnet_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<Persona> {
        self.create_and_save_new_mainnet_persona_with_factor_source_with_derivation_outcome(factor_source, name).await.map(|(x, _)| x)
    }

    pub async fn create_and_save_new_mainnet_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        self.create_and_save_new_persona_with_factor_source_with_derivation_outcome(
            factor_source,
            NetworkID::Mainnet,
            name,
        )
        .await
    }

    /// Create a new Persona and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_persona_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Persona> {
        let bdfs = self.bdfs()?;
        self.create_and_save_new_persona_with_factor_source(
            bdfs.into(),
            network_id,
            name,
        )
        .await
    }

    /// Create a new Persona and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Persona> {
        self.create_and_save_new_persona_with_factor_source_with_derivation_outcome(factor_source, network_id, name).await.map(|(x, _)| x)
    }

    pub async fn create_and_save_new_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        debug!("Creating persona.");
        let (persona, instances_consumer, derivation_outcome) = self
            .create_unsaved_persona_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                name,
            )
            .await?;
        debug!("Created persona, now saving it to profile.");

        // First try save Persona into Profile...
        self.add_persona(persona.clone()).await?;
        // ... if success, then delete FactorInstance from cache!
        instances_consumer.consume().await?;

        info!(
            "Created persona and saved new persona into profile, address: {}",
            persona.address
        );
        Ok((persona, derivation_outcome))
    }

    /// Creates persona using BDFS
    /// The persona names will be `<name_prefix> <index>`
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn batch_create_many_personas_with_bdfs_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<()> {
        self.batch_create_many_personas_with_bdfs_with_derivation_outcome_then_save_once(count, network_id, name_prefix).await.map(|_|{})
    }

    pub async fn batch_create_many_personas_with_bdfs_with_derivation_outcome_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<FactorInstancesProviderOutcomeForFactor> {
        let bdfs = self.bdfs()?;
        self.batch_create_many_personas_with_factor_source_with_derivation_outcome_then_save_once(
            bdfs.into(),
            count,
            network_id,
            name_prefix,
        )
        .await
    }

    /// Creates persona using specified factor source.
    /// The persona names will be `<name_prefix> <index>`
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn batch_create_many_personas_with_factor_source_then_save_once(
        &self,
        factor_source: FactorSource,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<()> {
        self.batch_create_many_personas_with_factor_source_with_derivation_outcome_then_save_once(factor_source, count, network_id, name_prefix).await.map(|_|{})
    }
    pub async fn batch_create_many_personas_with_factor_source_with_derivation_outcome_then_save_once(
        &self,
        factor_source: FactorSource,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<FactorInstancesProviderOutcomeForFactor> {
        debug!("Batch creating #{} personas.", count);
        let (personas, instances_consumer, derivation_outcome) = self
            .batch_create_unsaved_personas_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                count,
                name_prefix,
            )
            .await?;
        debug!("Created #{} personas, now saving them to profile.", count);

        // First save personas into Profile...
        self.add_personas(personas).await?;
        // ... if successful, consume FactorInstances from cache!
        instances_consumer.consume().await?;

        info!(
            "Created persona and saved #{} new personas into profile",
            count
        );
        Ok(derivation_outcome)
    }

    /// Creates many new non securified personas **WITHOUT** add them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add them to Profile, call `add_personas(personas)`
    ///
    /// # Emits Event
    /// Emits `Event::FactorSourceUpdated { id: FactorSourceID }` since the date in
    /// `factor_source.common.last_used` is updated.
    pub async fn batch_create_unsaved_personas_with_bdfs(
        &self,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<(Personas, InstancesConsumer)> {
        let bdfs = self.bdfs()?;
        self.batch_create_unsaved_personas_with_factor_source(
            bdfs.into(),
            network_id,
            count,
            name_prefix,
        )
        .await
    }

    /// Creates many new non securified personas **WITHOUT** add them to Profile, using selected factor source
    ///  and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add them to Profile, call `add_personas(personas)`
    ///
    /// # Emits Event
    /// Emits `Event::FactorSourceUpdated { id: FactorSourceID }` since the date in
    /// `factor_source.common.last_used` is updated.
    pub async fn batch_create_unsaved_personas_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<(Personas, InstancesConsumer)> {
        self.batch_create_unsaved_personas_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            count,
            name_prefix,
        )
        .await
        .map(|(x, y, _)| (x, y))
    }
    pub async fn batch_create_unsaved_personas_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<(
        Personas,
        InstancesConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let key_derivation_interactors = self.keys_derivation_interactors();

        let profile = self.profile()?;

        let (
            factor_source_id,
            personas,
            instances_consumer,
            derivation_outcome,
        ) = profile
            .create_unsaved_personas_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                count,
                Arc::new(self.clients.factor_instances_cache.clone()),
                key_derivation_interactors,
                |idx| {
                    DisplayName::new(format!("{} {}", name_prefix, idx))
                        .expect("Should not use a long name_prefix")
                },
            )
            .await?;

        // TODO: move this to the FactorInstancesProvider... it should take a `emit_last_used` closure
        // Change of `last_used_on` of FactorSource
        self.update_last_used_of_factor_source(factor_source_id)
            .await?;

        Ok((personas, instances_consumer, derivation_outcome))
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
        let address = persona.address;

        debug!("Adding persona with address: {} to profile", address);

        self.add_personas_without_emitting_persona_added_event(Personas::just(
            persona,
        ))
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::PersonaAdded { address },
            ))
            .await;

        Ok(())
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
        let addresses = personas
            .clone()
            .into_iter()
            .map(|a| a.address)
            .collect_vec();

        self.add_personas_without_emitting_persona_added_event(personas)
            .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::PersonasAdded { addresses },
            ))
            .await;

        Ok(())
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
        self.update_profile_with(|p| {
            if p.update_persona(&updated.address, |old| *old = updated.clone())
                .is_none()
            {
                Err(CommonError::UnknownPersona)
            } else {
                Ok(())
            }
        })
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::PersonaUpdated {
                    address: updated.address,
                },
            ))
            .await;

        Ok(())
    }
}

impl SargonOS {
    /// Adds the `personas` to active profile and **saves** the updated profile to
    /// secure storage, without emitting `Event::PersonaAdded`, but we DO emit
    /// `Event::ProfileSaved`.`
    ///
    /// Returns `Ok(())` if the `personas` were new and successfully added. If
    /// saving failed or if the personas were already present in Profile, an
    /// error is returned.
    ///
    /// # Emits
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    async fn add_personas_without_emitting_persona_added_event(
        &self,
        personas: Personas,
    ) -> Result<()> {
        if personas.is_empty() {
            warn!("Tried to add empty personas...");
            return Ok(());
        }

        let number_of_personas_to_add = personas.len();

        let network_id = personas
            .assert_elements_on_same_network()?
            .expect("Should have handled empty personas case already.");

        debug!("Adding #{} personas to Profile Network with ID: {} - or creating a Profile Network if it does not exist", number_of_personas_to_add, network_id);

        self.update_profile_with(|p| {
            let networks = &mut p.networks;

            if networks.contains_id(network_id) {
                debug!("Profile already contained network to add #{} persona(s) to, network_id: {}", number_of_personas_to_add, network_id);
                networks
                    .try_try_update_with(&network_id, |network| {
                        let count_before = network.personas.len();
                        debug!("Profile Network to add #{} persona(s) to contains #{} personas (before adding).", number_of_personas_to_add, count_before);
                        network.personas.extend(personas.clone());
                        let count_after = network.personas.len();
                        debug!("Profile Network now contains: #{} personas", count_after);
                        if network.personas.len() == count_before + number_of_personas_to_add {
                            Ok(())
                        } else {
                            Err(CommonError::UnableToAddAllPersonasDuplicatesFound)
                        }
                    })
            } else {
                debug!("No Profile Network exists with ID {}, creating it...", network_id);
                let network = ProfileNetwork::new(
                    network_id,
                    Accounts::default(),
                    personas.clone(),
                    AuthorizedDapps::default(),
                    ResourcePreferences::default(),
                );
                networks.append(network);
                Ok(())
            }
        })
        .await
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
                x.create_unsaved_mainnet_persona_with_bdfs(
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
    async fn test_create_unsaved_persona_twice_yield_same_personas_if_instances_consumer_is_not_used(
    ) {
        // ARRANGE
        let os = SUT::fast_boot_bdfs(MnemonicWithPassphrase::sample()).await;

        // ACT
        let (first, instances_consumer) = os
            .with_timeout(|x| {
                x.create_unsaved_unnamed_mainnet_persona_with_bdfs()
            })
            .await
            .unwrap();

        // Not used!
        drop(instances_consumer);

        let (second, _) = os
            .with_timeout(|x| {
                x.create_unsaved_unnamed_mainnet_persona_with_bdfs()
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
        let (first, instances_consumer) = os
            .with_timeout(|x| {
                x.create_unsaved_unnamed_mainnet_persona_with_bdfs()
            })
            .await
            .unwrap();

        // Consume!
        instances_consumer.consume().await.unwrap();

        let (second, _) = os
            .with_timeout(|x| {
                x.create_unsaved_unnamed_mainnet_persona_with_bdfs()
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
                x.create_and_save_new_unnamed_mainnet_persona_with_bdfs()
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
    async fn test_first_create_and_add_persona_has_index_0() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let persona = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            persona
                .security_state
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
                x.create_and_save_new_unnamed_mainnet_persona_with_bdfs()
            })
            .await
            .unwrap();

        let second = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_persona_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            second
                .security_state
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
        let n: u32 = 3;
        os.with_timeout(|x| {
            x.batch_create_many_personas_with_bdfs_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let indices = os.profile().unwrap().networks[0]
            .personas
            .iter()
            .map(|x| {
                x.security_state
                    .into_unsecured()
                    .unwrap()
                    .transaction_signing
                    .derivation_path()
                    .index()
            })
            .map(|i| u32::from(i.index_in_local_key_space()))
            .collect_vec();
        assert_eq!(indices, (0u32..n).collect_vec());
    }

    #[actix_rt::test]
    async fn test_batch_create_and_add_persona_n_has_names_with_index_appended_to_prefix(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n: u32 = 3;
        os.with_timeout(|x| {
            x.batch_create_many_personas_with_bdfs_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let names = os.profile().unwrap().networks[0]
            .personas
            .iter()
            .map(|x| x.display_name.value.clone())
            .collect_vec();

        assert_eq!(
            names,
            ["test 0", "test 1", "test 2"]
                .into_iter()
                .map(|x| x.to_owned())
                .collect_vec()
        );
    }

    #[actix_rt::test]
    async fn batch_create_persona_then_n_personas_are_saved_and_have_persona_data_default(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n = AppearanceID::all().len() as u32 * 2;
        os.with_timeout(|x| {
            x.batch_create_many_personas_with_bdfs_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        os.profile().unwrap().networks[0]
            .personas
            .iter()
            .for_each(|p| assert_eq!(p.persona_data, PersonaData::default()))
    }

    #[actix_rt::test]
    async fn batch_create_persona_unsaved_are_not_saved() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| {
            x.batch_create_unsaved_personas_with_bdfs(
                NetworkID::Mainnet,
                3,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        assert!(os.profile().unwrap().networks[0].personas.is_empty())
    }

    #[actix_rt::test]
    async fn test_create_unsaved_persona_emits_factor_source_updated() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap();
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        // ACT
        os.with_timeout(|x| {
            x.create_unsaved_unnamed_mainnet_persona_with_bdfs()
        })
        .await
        .unwrap();

        // ASSERT
        assert!(event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::FactorSourceUpdated));
    }

    #[actix_rt::test]
    async fn test_create_and_save_new_persona_emits_events() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap();

        // ACT
        os.with_timeout(|x| x.new_wallet()).await.unwrap();
        os.with_timeout(|x| {
            x.create_and_save_new_persona_with_bdfs(
                NetworkID::Mainnet,
                DisplayName::sample(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let events = event_bus_driver
            .recorded()
            .iter()
            .map(|e| e.event.kind())
            .collect_vec();

        use EventKind::*;
        assert_eq!(
            events,
            vec![
                Booted,
                ProfileSaved, // Save of initial profile
                ProfileSaved, // Save of the new persona
                FactorSourceUpdated,
                ProfileSaved,
                PersonaAdded
            ]
        );
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
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
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
                x.create_and_save_new_unnamed_mainnet_persona_with_bdfs()
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
                x.create_and_save_new_unnamed_mainnet_persona_with_bdfs()
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
                x.create_and_save_new_unnamed_mainnet_persona_with_bdfs()
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
                x.create_and_save_new_unnamed_mainnet_persona_with_bdfs()
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
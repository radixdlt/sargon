#![allow(non_snake_case)]

use crate::prelude::*;

#[cfg(debug_assertions)]
impl SargonOS {
    pub async fn with_bdfs() -> (Arc<Self>, FactorSource) {
        let os = Self::fast_boot().await;
        let bdfs = os.bdfs();
        (os, bdfs.into())
    }

    pub fn bdfs(&self) -> DeviceFactorSource {
        self.profile()
            .unwrap()
            .device_factor_sources()
            .first()
            .unwrap()
            .clone()
    }

    pub async fn create_and_save_new_account_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let bdfs = self.bdfs();
        self.create_and_save_new_account_with_factor_source(
            bdfs.into(),
            network_id,
            name,
        )
        .await
    }

    /// Create a new Persona with main BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_persona_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
        persona_data: Option<PersonaData>,
    ) -> Result<Persona> {
        let bdfs = self.bdfs();
        self.create_and_save_new_persona_with_factor_source(
            bdfs.into(),
            network_id,
            name,
            persona_data,
        )
        .await
    }

    pub async fn create_unsaved_account_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let bdfs = self.bdfs();
        self.create_unsaved_account_with_factor_source(
            bdfs.into(),
            network_id,
            name,
        )
        .await
    }

    pub async fn create_and_save_new_mainnet_account_with_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_and_save_new_account_with_bdfs(NetworkID::Mainnet, name)
            .await
    }

    pub async fn create_and_save_new_unnamed_mainnet_account_with_bdfs(
        &self,
    ) -> Result<Account> {
        self.create_and_save_new_mainnet_account_with_bdfs(
            DisplayName::sample(),
        )
        .await
    }

    pub async fn create_and_save_new_mainnet_account_with_derivation_outcome(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_mainnet_account_with_bdfs_with_derivation_outcome(display_name).await
    }

    pub async fn create_and_save_new_mainnet_persona(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Persona> {
        self.create_and_save_new_mainnet_persona_with_derivation_outcome(name)
            .await
            .map(|(p, _)| p)
    }

    pub async fn create_and_save_new_mainnet_account(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Account> {
        self.create_and_save_new_mainnet_account_with_derivation_outcome(name)
            .await
            .map(|(a, _)| a)
    }

    pub async fn create_and_save_new_persona_with_factor_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_persona_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name, None).await
    }

    pub async fn create_and_save_new_account_with_factor_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_account_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name).await
    }

    pub async fn create_and_save_new_mainnet_persona_with_derivation_outcome(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_mainnet_persona_with_bdfs_with_derivation_outcome(display_name).await
    }

    pub async fn create_and_save_new_mainnet_account_with_bdfs_with_derivation_outcome(
        &self,
        name: DisplayName,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let bdfs = self.bdfs();
        self.create_and_save_new_mainnet_account_with_factor_source_with_derivation_outcome(
            bdfs.into(),
            name,
        )
        .await
    }

    /// Create a new mainnet Account using the selected factor source and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_mainnet_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_and_save_new_account_with_factor_source(
            factor_source,
            NetworkID::Mainnet,
            name,
        )
        .await
    }

    pub async fn create_and_save_new_mainnet_account_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        self.create_and_save_new_account_with_factor_source_with_derivation_outcome(
            factor_source,
            NetworkID::Mainnet,
            name,
        )
        .await
    }

    /// Creates a new unsaved mainnet account named "Unnamed {N}", where `N` is the
    /// index of the next account for the main BDFS.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_account_with_bdfs(
        &self,
    ) -> Result<Account> {
        let bdfs = self.bdfs();
        self.create_unsaved_unnamed_mainnet_account_with_factor_source(
            bdfs.into(),
        )
        .await
    }

    /// Creates a new unsaved mainnet account named "Unnamed {N}", where `N` is the
    /// index of the next account for the selected factor_source.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_account_with_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<Account> {
        self.create_unsaved_account_with_factor_source(
            factor_source,
            NetworkID::Mainnet,
            DisplayName::new("Unnamed").unwrap(),
        )
        .await
    }

    /// Uses `create_unsaved_account` specifying `NetworkID::Mainnet` using main BDFS.
    pub async fn create_unsaved_mainnet_account_with_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        let bdfs = self.bdfs();
        self.create_unsaved_mainnet_account_with_factor_source(
            bdfs.into(),
            name,
        )
        .await
    }

    /// Uses `create_unsaved_account` specifying `NetworkID::Mainnet` using
    /// the specified `factor_source`.
    pub async fn create_unsaved_mainnet_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_unsaved_account_with_factor_source(
            factor_source,
            NetworkID::Mainnet,
            name,
        )
        .await
    }

    /// Creates account using main BDFS.
    /// The account names will be `<name_prefix> <index>`
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn batch_create_many_accounts_with_bdfs_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<Accounts> {
        self.batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(count, network_id, name_prefix).await.map(|(x, _) |x)
    }

    pub async fn batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<(Accounts, FactorInstancesProviderOutcomeForFactor)> {
        let bdfs = self.bdfs();
        self.batch_create_many_accounts_with_factor_source_with_derivation_outcome_then_save_once(
            bdfs.into(),
            count,
            network_id,
            name_prefix,
        )
        .await
    }

    /// Creates account using specified factor source.
    /// The account names will be `<name_prefix> <index>`
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn batch_create_many_accounts_with_factor_source_with_derivation_outcome_then_save_once(
        &self,
        factor_source: FactorSource,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<(Accounts, FactorInstancesProviderOutcomeForFactor)> {
        debug!("Batch creating #{} accounts.", count);
        let (accounts, instances_in_cache_consumer, derivation_outcome) = self
            .batch_create_unsaved_accounts_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                count,
                name_prefix,
            )
            .await?;
        debug!("Created #{} accounts, requesting authorization...", count);

        let authorization = self
            .authorization_interactor()
            .request_authorization(AuthorizationPurpose::CreatingAccounts)
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

        // First try to save accounts into Profile...
        self.add_accounts(accounts.clone()).await?;
        // ... if successful consume the FactorInstances from the Cache!
        instances_in_cache_consumer.consume().await?;

        info!("Created and saved #{} new accounts into profile", count);
        Ok((accounts, derivation_outcome))
    }

    /// Creates many new non securified accounts **WITHOUT** add them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add them to Profile, call `add_accounts(accounts)`
    ///
    /// # Emits Event
    /// Emits `Event::FactorSourceUpdated { id: FactorSourceID }` since the date in
    /// `factor_source.common.last_used` is updated.
    pub async fn batch_create_unsaved_accounts_with_bdfs_consuming_factor_instances(
        &self,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<Accounts> {
        let (accounts, instances_in_cache_consumer) = self
            .batch_create_unsaved_accounts_with_bdfs(
                network_id,
                count,
                name_prefix,
            )
            .await?;
        instances_in_cache_consumer.consume().await?;
        Ok(accounts)
    }

    /// Creates many new non securified accounts **WITHOUT** add them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add them to Profile, call `add_accounts(accounts)`
    ///
    /// # Emits Event
    /// Emits `Event::FactorSourceUpdated { id: FactorSourceID }` since the date in
    /// `factor_source.common.last_used` is updated.
    pub async fn batch_create_unsaved_accounts_with_bdfs(
        &self,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<(Accounts, InstancesInCacheConsumer)> {
        let bdfs = self.bdfs();
        self.batch_create_unsaved_accounts_with_factor_source(
            bdfs.into(),
            network_id,
            count,
            name_prefix,
        )
        .await
    }

    /// Creates many new non securified accounts **WITHOUT** add them to Profile, using selected factor source
    ///  and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add them to Profile, call `add_accounts(accounts)`
    ///
    /// # Emits Event
    /// Emits `Event::FactorSourceUpdated { id: FactorSourceID }` since the date in
    /// `factor_source.common.last_used` is updated.
    pub async fn batch_create_unsaved_accounts_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<(Accounts, InstancesInCacheConsumer)> {
        self.batch_create_unsaved_accounts_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            count,
            name_prefix,
        )
        .await
        .map(|(x, y, _)| (x, y))
    }
    pub async fn batch_create_unsaved_accounts_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<(
        Accounts,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        self.batch_create_unsaved_entities_with_factor_source_with_derivation_outcome(factor_source, network_id, count, name_prefix)
        .await
        .map(|(a, b, c)| (a.into_iter().collect(), b, c))
    }

    pub async fn batch_create_unsaved_entities_with_factor_source_with_derivation_outcome<
        E: IsEntity,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<(
        IdentifiedVecOf<E>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let key_derivation_interactors = self.keys_derivation_interactor();

        let profile = self.profile()?;

        let (
            factor_source_id,
            entities,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = profile
            .create_unsaved_entities_with_factor_source_with_derivation_outcome(
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

        Ok((entities, instances_in_cache_consumer, derivation_outcome))
    }

    /// Creates a new unsaved mainnet persona named "Unnamed" using main bdfs.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_persona_with_bdfs(
        &self,
    ) -> Result<(Persona, InstancesInCacheConsumer)> {
        let bdfs = self.bdfs();
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
    ) -> Result<(Persona, InstancesInCacheConsumer)> {
        self.create_unsaved_persona_with_factor_source(
            factor_source,
            NetworkID::Mainnet,
            DisplayName::new("Unnamed").unwrap(),
        )
        .await
    }

    /// Uses `create_unsaved_persona` specifying `NetworkID::Mainnet` using main BDFS.
    pub async fn create_unsaved_mainnet_persona_with_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<(Persona, InstancesInCacheConsumer)> {
        let bdfs = self.bdfs();
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
    ) -> Result<(Persona, InstancesInCacheConsumer)> {
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
    ) -> Result<(Persona, InstancesInCacheConsumer)> {
        let bdfs = self.bdfs();
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
    ) -> Result<(Persona, InstancesInCacheConsumer)> {
        self.create_unsaved_persona_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            name,
        )
        .await
        .map(|(x, y, _)| (x, y))
    }

    /// Create a new mainnet Persona named "Unnamed" using main BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_unnamed_mainnet_persona_with_bdfs(
        &self,
    ) -> Result<Persona> {
        let bdfs = self.bdfs();
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

    /// Create a new mainnet Persona using the mian BDFS and adds it to the active Profile.
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
        let bdfs = self.bdfs();
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
            None,
        )
        .await
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
    ) -> Result<Personas> {
        self.batch_create_many_personas_with_bdfs_with_derivation_outcome_then_save_once(count, network_id, name_prefix).await.map(|(x, _) | x)
    }

    pub async fn batch_create_many_personas_with_bdfs_with_derivation_outcome_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<(Personas, FactorInstancesProviderOutcomeForFactor)> {
        let bdfs = self.bdfs();
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
    pub async fn batch_create_many_personas_with_factor_source_with_derivation_outcome_then_save_once(
        &self,
        factor_source: FactorSource,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<(Personas, FactorInstancesProviderOutcomeForFactor)> {
        debug!("Batch creating #{} personas.", count);
        let (personas, instances_in_cache_consumer, derivation_outcome) = self
            .batch_create_unsaved_personas_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                count,
                name_prefix,
            )
            .await?;
        debug!("Created #{} personas, requesting authorization...", count);

        let authorization = self
            .authorization_interactor()
            .request_authorization(AuthorizationPurpose::CreatingPersonas)
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

        // First save personas into Profile...
        self.add_personas(personas.clone()).await?;
        // ... if successful, consume FactorInstances from cache!
        instances_in_cache_consumer.consume().await?;

        info!("Created and saved #{} new personas into profile", count);
        Ok((personas, derivation_outcome))
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
    ) -> Result<(Personas, InstancesInCacheConsumer)> {
        let bdfs = self.bdfs();
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
    ) -> Result<(Personas, InstancesInCacheConsumer)> {
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
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let key_derivation_interactors = self.keys_derivation_interactor();

        let profile = self.profile()?;

        let (
            factor_source_id,
            personas,
            instances_in_cache_consumer,
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
                        .expect("Should have used a non empty name.")
                },
            )
            .await?;

        // TODO: move this to the FactorInstancesProvider... it should take a `emit_last_used` closure
        // Change of `last_used_on` of FactorSource
        self.update_last_used_of_factor_source(factor_source_id)
            .await?;

        Ok((personas, instances_in_cache_consumer, derivation_outcome))
    }
}

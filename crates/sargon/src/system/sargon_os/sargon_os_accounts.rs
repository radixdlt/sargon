use std::{borrow::Borrow, sync::RwLockWriteGuard};

use radix_common::address;

use crate::prelude::*;

// ==================
// Create Unsaved Account(s)
// ==================
impl SargonOS {
    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    pub fn accounts_on_current_network(&self) -> Result<Accounts> {
        self.profile_state_holder.accounts_on_current_network()
    }

    /// Returns the non-hidden accounts on the current network as `AccountForDisplay`
    pub fn accounts_for_display_on_current_network(
        &self,
    ) -> Result<AccountsForDisplay> {
        self.profile_state_holder
            .accounts_for_display_on_current_network()
    }

    /// Looks up the account by account address, returns Err if the account is
    /// unknown, will return a hidden account if queried for.
    pub fn account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<Account> {
        self.profile_state_holder.account_by_address(address)
    }

    /// Creates a new unsaved mainnet account named "Unnamed {N}", where `N` is the
    /// index of the next account for the BDFS.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_account_with_bdfs(
        &self,
    ) -> Result<Account> {
        let bdfs = self.bdfs()?;
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

    /// Uses `create_unsaved_account` specifying `NetworkID::Mainnet` using
    /// the specified `factor_source`.
    pub async fn create_unsaved_mainnet_account_with_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        let bdfs = self.bdfs()?;
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

    /// Creates a new non securified account **WITHOUT** adding it to Profile,
    /// using the *main* "Babylon" `DeviceFactorSource` and the "next" index for
    /// this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `os.add_account(account)`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage, since the `last_used_on` date
    /// of the factor source has been updated.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn create_unsaved_account_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let bdfs = self.bdfs()?;
        self.create_unsaved_account_with_factor_source(
            bdfs.into(),
            network_id,
            name,
        )
        .await
    }

    /// Creates a new non securified account **WITHOUT** adding it to Profile,
    /// using specified factor source and the "next" index for this FactorSource
    ///
    /// If you want to add it to Profile, call `os.add_account(account)`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage, since the `last_used_on` date
    /// of the factor source has been updated.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn create_unsaved_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_unsaved_account_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            name,
        )
        .await
        .map(|(x, _, _)| x)
    }

    pub async fn create_unsaved_account_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<(
        Account,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let key_derivation_interactors = self.keys_derivation_interactors();

        let profile = self.profile()?;
        let cache_client =
            Arc::new(self.clients.factor_instances_cache.clone());

        let future = profile
            .create_unsaved_account_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                name,
                cache_client,
                key_derivation_interactors,
            );

        let outcome = future.await?;
        let (
            factor_source_id,
            account,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = outcome;

        // TODO: move this to the FactorInstancesProvider... it should take a `emit_last_used` closure
        // Change of `last_used_on` of FactorSource
        self.update_last_used_of_factor_source(factor_source_id)
            .await?;

        Ok((account, instances_in_cache_consumer, derivation_outcome))
    }

    /// Create a new mainnet Account named "Unnamed" using BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_unnamed_mainnet_account_with_bdfs(
        &self,
    ) -> Result<Account> {
        let bdfs = self.bdfs()?;
        self.create_and_save_new_unnamed_mainnet_account_with_factor_source(
            bdfs.into(),
        )
        .await
    }

    /// Create a new mainnet Account named "Unnamed" using selected factor source and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_unnamed_mainnet_account_with_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<Account> {
        self.create_and_save_new_mainnet_account_with_factor_source(
            factor_source,
            DisplayName::new("Unnamed").unwrap(),
        )
        .await
    }

    /// Create a new mainnet Account using the BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_mainnet_account_with_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_and_save_new_mainnet_account_with_bdfs_with_derivation_outcome(name).await.map(|(x, _)| x)
    }

    pub async fn create_and_save_new_mainnet_account_with_bdfs_with_derivation_outcome(
        &self,
        name: DisplayName,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let bdfs = self.bdfs()?;
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
        self.create_and_save_new_mainnet_account_with_factor_source_with_derivation_outcome(factor_source, name).await.map(|(x, _)| x)
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

    /// Create a new Account and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_account_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let bdfs = self.bdfs()?;
        self.create_and_save_new_account_with_factor_source(
            bdfs.into(),
            network_id,
            name,
        )
        .await
    }

    /// Create a new Account and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_and_save_new_account_with_factor_source_with_derivation_outcome(factor_source, network_id, name).await.map(|(x, _)| x)
    }

    pub async fn create_and_save_new_account_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        debug!("Creating account.");
        let (account, instances_in_cache_consumer, derivation_outcome) = self
            .create_unsaved_account_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                name,
            )
            .await?;
        debug!("Created account, now saving it to profile.");

        // Add account to Profile...
        self.add_account(account.clone()).await?;
        // .. if successful consume the FactorInstances from the Cache!
        instances_in_cache_consumer.consume().await?;

        info!(
            "Created account and saved new account into profile, address: {}",
            account.address
        );
        Ok((account, derivation_outcome))
    }

    /// Creates account using BDFS
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
        let bdfs = self.bdfs()?;
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
        debug!("Created #{} accounts, now saving them to profile.", count);

        // First try to save accounts into Profile...
        self.add_accounts(accounts.clone()).await?;
        // ... if successful consume the FactorInstances from the Cache!
        instances_in_cache_consumer.consume().await?;

        info!(
            "Created account and saved #{} new accounts into profile",
            count
        );
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
        let bdfs = self.bdfs()?;
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
        self.batch_create_unsaved_entities_with_factor_source_with_derivation_outcome(factor_source, network_id, count, name_prefix).await
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
        let key_derivation_interactors = self.keys_derivation_interactors();

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
}

// ==================
// Add (Save) Account(s)
// ==================
impl SargonOS {
    /// Add the `account` to active profile and **saves** the updated profile to
    /// secure storage.
    ///
    /// Returns `Ok(())` if the `account` was new and successfully added. If
    /// saving failed or if the account was already present in Profile, an
    /// error is returned.
    ///
    /// # Emits Events
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::AccountsAdded { addresses } }`
    pub async fn add_account(&self, account: Account) -> Result<()> {
        self.add_entity(account).await
    }

    pub async fn add_entity<E: IsEntity>(&self, entity: E) -> Result<()> {
        let address = entity.address();
        debug!("Adding entity with address: {} to profile", address);
        self.add_entities(IdentifiedVecOf::just(entity)).await
    }

    /// Adds the `accounts` to active profile and **saves** the updated profile to
    /// secure storage.
    ///
    /// Returns `Ok(())` if the `accounts` were new and successfully added. If
    /// saving failed or if the accounts were already present in Profile, an
    /// error is returned.
    ///
    /// # Emits Events
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::AccountsAdded { addresses } }`
    pub async fn add_accounts(&self, accounts: Accounts) -> Result<()> {
        self.add_entities(accounts).await.map_err(|e| match e {
            CommonError::UnableToAddAllEntitiesDuplicatesFound => {
                CommonError::UnableToAddAllAccountsDuplicatesFound
            }
            _ => e,
        })
    }
}

// ==================
// Update Account(s)
// ==================
impl SargonOS {
    /// Updates the account `updated` by mutating current profile and persisting
    /// the change to secure storage. Throws `UnknownAccount` error if the account
    /// is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountUpdated { address } }`
    pub async fn update_account(&self, updated: Account) -> Result<()> {
        self.update_entity(updated).await
    }

    /// Updates the accounts `updated` by mutating current profile and persisting
    /// the change to secure storage. Throws `UnknownAccount` error if any of the account
    /// is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountsUpdated { addresses } }`
    pub async fn update_accounts(&self, updated: Accounts) -> Result<()> {
        self.update_entities(updated).await
    }

    pub async fn update_entity<E: IsEntity>(&self, updated: E) -> Result<()> {
        self.update_entities(IdentifiedVecOf::just(updated)).await
    }

    pub async fn update_entities<E: IsEntity>(
        &self,
        updated: IdentifiedVecOf<E>,
    ) -> Result<()> {
        let addresses = updated
            .clone()
            .into_iter()
            .map(|e| e.address())
            .collect::<IndexSet<_>>();
        self.update_profile_with(|p| p.update_entities(updated.clone()))
            .await?;

        if let Some(event) = E::profile_modified_event(true, addresses) {
            self.event_bus
                .emit(EventNotification::profile_modified(event))
                .await;
        }
        Ok(())
    }

    /// Updates the profile by marking the account with `account_address` as hidden.
    pub async fn mark_account_as_hidden(
        &self,
        account_address: AccountAddress,
    ) -> Result<()> {
        self.update_profile_with(|profile| {
            profile.networks.hide_account(&account_address);
            Ok(())
        })
        .await
    }

    /// Updates the profile by marking the account with `account_address` as tombstoned.
    pub async fn mark_account_as_tombstoned(
        &self,
        account_address: AccountAddress,
    ) -> Result<()> {
        self.update_profile_with(|profile| {
            profile.networks.tombstone_account(&account_address);
            Ok(())
        })
        .await
    }

    /// Updates the profile by marking the account with `account_addresses` as tombstoned.
    pub async fn mark_accounts_as_tombstoned(
        &self,
        account_addresses: Vec<AccountAddress>,
    ) -> Result<()> {
        self.update_profile_with(move |profile| {
            profile.networks.tombstone_accounts(&account_addresses);
            Ok(())
        })
        .await
    }
}

impl<E: IsEntity> IdentifiedVecOf<E> {
    pub fn to_accounts(self) -> Result<Accounts> {
        self.into_iter()
            .map(|e| {
                Account::try_from(Into::<AccountOrPersona>::into(e.clone()))
                    .map_err(|_| CommonError::ExpectedAccountButGotPersona {
                        address: e.address().to_string(),
                    })
            })
            .collect::<Result<Accounts>>()
    }

    pub fn to_personas(self) -> Result<Personas> {
        self.into_iter()
            .map(|e| {
                Persona::try_from(Into::<AccountOrPersona>::into(e.clone()))
                    .map_err(|_| CommonError::ExpectedPersonaButGotAccount {
                        address: e.address().to_string(),
                    })
            })
            .collect::<Result<Personas>>()
    }
}

impl SargonOS {
    /// Adds the `entities` to active profile and **saves** the updated profile to
    /// secure storage, without emitting any `Event`, but we DO emit
    /// `Event::ProfileSaved`.`
    ///
    /// Returns `Ok(())` if the `entities` were new and successfully added. If
    /// saving failed or if the entities were already present in Profile, an
    /// error is returned.
    ///
    /// # Emits
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn add_entities<E: IsEntity>(
        &self,
        entities: IdentifiedVecOf<E>,
    ) -> Result<()> {
        if entities.is_empty() {
            warn!("Tried to add empty entities...");
            return Ok(());
        }

        let entity_kind = E::entity_kind();
        let number_of_entities_to_add = entities.len();

        let network_id = entities
            .assert_elements_on_same_network()?
            .expect("Should have handled empty entities case already.");

        debug!("Adding #{} entities to Profile Network with ID: {} - or creating a Profile Network if it does not exist", number_of_entities_to_add, network_id);

        let to_accounts =
            || -> Accounts { entities.clone().to_accounts().unwrap() };

        let to_personas =
            || -> Personas { entities.clone().to_personas().unwrap() };

        self.update_profile_with(|p| {
            let networks = &mut p.networks;
            let networks_backup = networks.clone();
            if networks.contains_id(network_id) {
                debug!("Profile already contained network to add #{} entities to, network_id: {}", number_of_entities_to_add, network_id);
                networks
                    .try_try_update_with(&network_id, |network| {
                        let count_before = match entity_kind {
                            CAP26EntityKind::Account => network.accounts.len(),
                            CAP26EntityKind::Identity => network.personas.len(),
                        };
                        debug!("Profile Network to add #{} entities to contains #{} entities (before adding).", number_of_entities_to_add, count_before);

                        match entity_kind {
                            CAP26EntityKind::Account => {
                                network.accounts.extend(to_accounts());
                            }
                            CAP26EntityKind::Identity => {
                                network.personas.extend(to_personas());
                            }
                        }

                        let count_after = match entity_kind {
                            CAP26EntityKind::Account => network.accounts.len(),
                            CAP26EntityKind::Identity => network.personas.len(),
                        };
                        debug!("Profile Network now contains: #{} entities", count_after);

                        if count_after == count_before + number_of_entities_to_add {
                            Ok(())
                        } else {
                            Err(CommonError::UnableToAddAllEntitiesDuplicatesFound)
                        }
                    })
            } else {
                debug!("No Profile Network exists with ID {}, creating it...", network_id);
                let network = match entity_kind {
                    CAP26EntityKind::Account => {
                        ProfileNetwork::new(
                            network_id,
                            to_accounts(),
                            Personas::default(),
                            AuthorizedDapps::default(),
                            ResourcePreferences::default(),
                        )
                    }
                    CAP26EntityKind::Identity => {
                        ProfileNetwork::new(
                            network_id,
                            Accounts::default(),
                            to_personas(),
                            AuthorizedDapps::default(),
                            ResourcePreferences::default(),
                        )
                    }
                };

                networks.append(network);
                Ok(())
            }
            .and_then(|_| {
                p.assert_new_factor_instances_not_already_used(entities.clone()).inspect_err(|_| { p.networks = networks_backup; })
            })
        })
        .await?;

        if let Some(event) = E::profile_modified_event(
            false,
            entities.clone().into_iter().map(|e| e.address()).collect(),
        ) {
            self.event_bus
                .emit(EventNotification::profile_modified(event))
                .await;
        }

        Ok(())
    }
}

// # Securify
impl SargonOS {
    #[allow(dead_code)]
    #[cfg(test)]
    pub(crate) async fn make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome<
        A: IsEntityAddress,
    >(
        &self,
        addresses_of_entities: IndexSet<A>,
        security_structure_of_factor_sources: SecurityStructureOfFactorSources, // Aka "shield"
    ) -> Result<(
        IndexMap<A, SecurityStructureOfFactorInstances>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcome,
    )> {
        let profile_snapshot = self.profile()?;
        let key_derivation_interactors = self.keys_derivation_interactors();
        let matrix_of_factor_sources =
            &security_structure_of_factor_sources.matrix_of_factors;

        let (instances_in_cache_consumer, outcome) =
            SecurifyEntityFactorInstancesProvider::for_entity_mfa::<A>(
                Arc::new(self.clients.factor_instances_cache.clone()),
                Arc::new(profile_snapshot.clone()),
                matrix_of_factor_sources.clone(),
                addresses_of_entities.clone(),
                key_derivation_interactors,
            )
            .await?;

        let mut instances_per_factor_source = outcome
            .clone()
            .per_factor
            .into_iter()
            .map(|(k, outcome_per_factor)| {
                (k, outcome_per_factor.to_use_directly)
            })
            .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>();

        assert_eq!(
            instances_per_factor_source
                .keys()
                .cloned()
                .collect::<HashSet<FactorSourceIDFromHash>>(),
            matrix_of_factor_sources
                .all_factors()
                .into_iter()
                .map(|f| f.id_from_hash())
                .collect::<HashSet<FactorSourceIDFromHash>>()
        );

        let security_structure_id = security_structure_of_factor_sources.id();

        let security_structures_of_factor_instances = addresses_of_entities.clone().into_iter().map(|entity_address|
        {
            let security_structure_of_factor_instances: SecurityStructureOfFactorInstances = {
               let matrix_of_factor_instances = MatrixOfFactorInstances::fulfilling_matrix_of_factor_sources_with_instances(
                &mut instances_per_factor_source,
                matrix_of_factor_sources.clone(),
               )?;
                SecurityStructureOfFactorInstances::new(
                    security_structure_id,
                    matrix_of_factor_instances
                )
            };
            Ok((entity_address, security_structure_of_factor_instances))
        }).collect::<Result<IndexMap<A, SecurityStructureOfFactorInstances>>>()?;

        Ok((
            security_structures_of_factor_instances,
            instances_in_cache_consumer,
            outcome,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use futures::future::Shared;
    use std::{future::join, future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_first_add_account() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.add_account(Account::sample()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.profile().unwrap().networks[0].accounts.len(), 1);
    }

    #[actix_rt::test]
    async fn test_content_hint_is_updated_when_accounts_are_added() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.add_account(Account::sample()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile()
                .unwrap()
                .header
                .content_hint
                .number_of_accounts_on_all_networks_in_total,
            1
        );
        assert_eq!(
            os.profile().unwrap().header.content_hint.number_of_networks,
            1
        );
    }

    #[actix_rt::test]
    async fn test_first_create_unsaved_account() {
        // ARRANGE
        let os = SUT::fast_boot_bdfs(MnemonicWithPassphrase::sample()).await;

        // ACT
        let unsaved_account = os
            .with_timeout(|x| {
                x.create_unsaved_mainnet_account_with_bdfs(
                    DisplayName::new("Alice").unwrap(),
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(unsaved_account, Account::sample());
        assert_eq!(os.profile().unwrap().networks[0].accounts.len(), 0); // not added
    }

    #[actix_rt::test]
    async fn test_create_unsaved_account_twice_yield_same_accounts() {
        // ARRANGE
        let os = SUT::fast_boot_bdfs(MnemonicWithPassphrase::sample()).await;

        // ACT
        let first = os
            .with_timeout(|x| {
                x.create_unsaved_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        let second = os
            .with_timeout(|x| {
                x.create_unsaved_account_with_bdfs(
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
    async fn test_first_create_and_add_account_is_added() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile().unwrap().networks[0].accounts,
            Accounts::just(account)
        );
    }

    #[actix_rt::test]
    async fn test_first_create_and_add_account_has_index_0() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            account
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
    async fn test_second_create_and_add_account_has_index_1() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let _ = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        let second = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
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
    async fn batch_create_account_then_n_accounts_are_saved_and_have_indices_0_through_n(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n: u32 = 3;
        os.with_timeout(|x| {
            x.batch_create_many_accounts_with_bdfs_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let indices = os.profile().unwrap().networks[0]
            .accounts
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
    async fn test_batch_create_unsaved_accounts_with_bdfs_consuming_factor_instances(
    ) {
        // ARRANGE
        let os = SUT::fast_boot_bdfs(MnemonicWithPassphrase::sample()).await;

        // ACT
        let network = NetworkID::Mainnet;
        let n = 50;
        let accounts = os
            .with_timeout(|x| {
                x.batch_create_unsaved_accounts_with_bdfs_consuming_factor_instances(
                    network,
                    n as u16,
                    "acco".to_owned()
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(accounts.len(), n);
        assert_eq!(
            HashSet::from_iter(
                accounts.clone().into_iter().map(|a| a.address())
            )
            .len(),
            n
        );

        // First Accounts derivation entity index
        assert_eq!(
            accounts
                .first()
                .unwrap()
                .try_get_unsecured_control()
                .unwrap()
                .transaction_signing
                .derivation_entity_index(),
            HDPathComponent::from_local_key_space(
                0,
                KeySpace::Unsecurified { is_hardened: true },
            )
            .unwrap(),
        );

        // Last Accounts derivation entity index
        assert_eq!(
            accounts
                .items()
                .into_iter()
                .last()
                .unwrap()
                .try_get_unsecured_control()
                .unwrap()
                .transaction_signing
                .derivation_entity_index(),
            HDPathComponent::from_local_key_space(
                (n as u32) - 1,
                KeySpace::Unsecurified { is_hardened: true },
            )
            .unwrap(),
        );
    }

    #[actix_rt::test]
    async fn test_batch_create_and_add_account_n_has_names_with_index_appended_to_prefix(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n: u32 = 3;
        os.with_timeout(|x| {
            x.batch_create_many_accounts_with_bdfs_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let names = os.profile().unwrap().networks[0]
            .accounts
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
    async fn batch_create_account_then_n_accounts_are_saved_and_have_appearance_id_0_through_max(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n = AppearanceID::all().len() as u32 * 2;
        os.with_timeout(|x| {
            x.batch_create_many_accounts_with_bdfs_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let appearance_ids = os.profile().unwrap().networks[0]
            .accounts
            .iter()
            .map(|x| x.appearance_id)
            .collect_vec();

        assert_eq!(
            appearance_ids,
            [AppearanceID::all(), AppearanceID::all()].concat()
        );
    }

    #[actix_rt::test]
    async fn batch_create_account_unsaved_are_not_saved() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| {
            x.batch_create_unsaved_accounts_with_bdfs(
                NetworkID::Mainnet,
                3,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        assert!(os.profile().unwrap().networks[0].accounts.is_empty())
    }

    #[actix_rt::test]
    async fn test_create_unsaved_account_emits_factor_source_updated() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap();
        os.with_timeout(|x| x.new_wallet(false)).await.unwrap();

        // ACT
        os.with_timeout(|x| {
            x.create_unsaved_unnamed_mainnet_account_with_bdfs()
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
    async fn test_create_and_save_new_account_emits_events() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap();

        // ACT
        os.with_timeout(|x| x.new_wallet(false)).await.unwrap();
        os.with_timeout(|x| {
            x.create_and_save_new_account_with_bdfs(
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
                ProfileSaved, // Save of the new account
                FactorSourceUpdated,
                ProfileSaved,
                AccountAdded
            ]
        );
    }

    #[actix_rt::test]
    async fn update_account_updates_in_memory_profile() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let mut account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        account.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_account(account.clone()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile().unwrap().networks[0].accounts[0],
            account.clone()
        )
    }

    #[actix_rt::test]
    async fn update_account_updates_saved_profile() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let mut account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        account.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_account(account.clone()))
            .await
            .unwrap();

        // ASSERT
        let saved_profile = os
            .with_timeout(|x| x.secure_storage.load_profile())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(saved_profile.networks[0].accounts[0], account.clone())
    }

    #[actix_rt::test]
    async fn test_update_account_emits() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap();
        os.with_timeout(|x| x.new_wallet(false)).await.unwrap();

        let mut account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        account.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_account(account.clone()))
            .await
            .unwrap();

        // ASSERT
        assert!(event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::AccountUpdated));
    }

    #[actix_rt::test]
    async fn update_account_unknown_accounts_throws() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let result = os
            .with_timeout(|x| x.update_account(Account::sample()))
            .await;

        // ASSERT
        assert_eq!(result, Err(CommonError::UnknownAccount))
    }

    #[actix_rt::test]
    async fn add_accounts_empty_is_ok() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let result = os.with_timeout(|x| x.add_accounts(Accounts::new())).await;

        // ASSERT
        assert!(result.is_ok())
    }

    #[actix_rt::test]
    async fn add_accounts_duplicates_throws() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        let result = os
            .with_timeout(|x| x.add_accounts(Accounts::just(account.clone())))
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::UnableToAddAllAccountsDuplicatesFound)
        )
    }

    #[actix_rt::test]
    async fn add_account_new_network_works() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        assert_eq!(os.profile().unwrap().networks.len(), 1);

        // ACT
        os.with_timeout(|x| {
            x.add_accounts(Accounts::just(Account::sample_stokenet()))
        })
        .await
        .unwrap();

        // ASSERT
        assert_eq!(os.profile().unwrap().networks.len(), 2);
    }

    #[actix_rt::test]
    async fn add_account_new_network_but_same_factor_instance_as_existing_throws(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let account = Account::sample();
        let fsid = account
            .try_get_unsecured_control()
            .unwrap()
            .transaction_signing
            .factor_source_id
            .to_string();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        let mut account_same_fi_new_network = account.clone();
        let other_network = NetworkID::Stokenet;
        account_same_fi_new_network.address =
            account.address().map_to_network(other_network);
        account_same_fi_new_network.network_id = other_network;

        assert_eq!(
            account_same_fi_new_network.address.network_id(),
            other_network
        );
        assert_eq!(
            account_same_fi_new_network.address.node_id(),
            account.address().node_id()
        );

        let profile_snapshot_before_failing_op = os.profile().unwrap();
        let res = os
            .with_timeout(|x| {
                x.add_accounts(Accounts::just(
                    account_same_fi_new_network.clone(),
                ))
            })
            .await;

        // ASSERT
        assert!(res.is_err());

        let err = CommonError::FactorInstancesDiscrepancy {
            address_of_entity1: account.address().to_string(),
            address_of_entity2: account_same_fi_new_network
                .address()
                .to_string(),
            factor_source_id: fsid,
        };
        pretty_assertions::assert_eq!(res, Err(err));
        let profile_snapshot_after_failing_op = os.profile().unwrap();
        assert_eq!(
            profile_snapshot_after_failing_op,
            profile_snapshot_before_failing_op
        );
    }

    #[actix_rt::test]
    async fn test_accounts_on_current_network_empty() {
        let os = SUT::fast_boot().await;
        assert_eq!(os.accounts_on_current_network().unwrap(), Accounts::new());
    }

    #[actix_rt::test]
    async fn test_accounts_on_current_network_non_empty() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.accounts_on_current_network().unwrap(),
            Accounts::just(account)
        );
    }

    #[actix_rt::test]
    async fn test_accounts_on_current_network_empty_when_switched_network() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let _ = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        // ACT
        let _ = os
            .with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.accounts_on_current_network().unwrap(), Accounts::new());
    }

    #[actix_rt::test]
    async fn test_accounts_for_display_on_current_network() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.accounts_for_display_on_current_network().unwrap(),
            AccountsForDisplay::just(AccountForDisplay::from(account))
        );
    }

    #[actix_rt::test]
    async fn test_account_by_address_exists() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.account_by_address(account.address), Ok(account));
    }

    #[actix_rt::test]
    async fn test_account_by_address_not_exists() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        // so that we have at least one network (with one account)
        let _ = os
            .with_timeout(|x| {
                x.create_and_save_new_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.account_by_address(AccountAddress::sample_mainnet()),
            Err(CommonError::UnknownAccount)
        );
    }

    #[actix_rt::test]
    async fn test_mark_account_as_hidden_becomes_hidden() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let account = Account::sample_mainnet();

        let authorized_dapps: AuthorizedDapps = serde_json::from_str(r#"
        [
			{
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
				"displayName": "Radix Dashboard",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 2
							},
							"ids": [
								"account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					},
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 2
							},
							"ids": [
                                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					}
				]
			},
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				"displayName": "Gumball Club",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 2
							},
							"ids": [
                                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					}
				]
			}
        ]
            "#
        ).unwrap();

        // ACT
        // so that we have at least one network (with one account)
        os.with_timeout(|os| os.add_account(account.clone()))
            .await
            .unwrap();

        os.with_timeout(|os| {
            os.update_profile_with(|profile| {
                profile.networks.update_with(NetworkID::Mainnet, |network| {
                    network.authorized_dapps = authorized_dapps.clone();
                });
                Ok(())
            })
        })
        .await
        .unwrap();

        os.mark_account_as_hidden(account.address).await.unwrap();

        // ASSERT
        assert!(os.account_by_address(account.address).unwrap().is_hidden());

        let expected_authorized_dapps: AuthorizedDapps = serde_json::from_str(r#"
			[
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
				"displayName": "Radix Dashboard",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 2
							},
							"ids": [
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					},
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 2
							},
							"ids": [
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					}
				]
			},
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				"displayName": "Gumball Club",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 2
							},
							"ids": [
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					}
				]
			}
        ]
            "#
        ).unwrap();

        let updated_authorized_dapps = os
            .profile()
            .unwrap()
            .clone()
            .current_network()
            .unwrap()
            .authorized_dapps
            .clone();
        pretty_assertions::assert_eq!(
            updated_authorized_dapps,
            expected_authorized_dapps
        )
    }

    #[actix_rt::test]
    async fn test_mark_account_as_tombstoned_becomes_tombstoned() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let account = Account::sample_mainnet();

        let authorized_dapps: AuthorizedDapps = serde_json::from_str(r#"
        [
			{
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
				"displayName": "Radix Dashboard",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 2
							},
							"ids": [
								"account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					},
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 2
							},
							"ids": [
                                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					}
				]
			},
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				"displayName": "Gumball Club",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 2
							},
							"ids": [
                                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					}
				]
			}
        ]
            "#
        ).unwrap();

        // ACT
        // so that we have at least one network (with one account)
        os.with_timeout(|os| os.add_account(account.clone()))
            .await
            .unwrap();

        os.with_timeout(|os| {
            os.update_profile_with(|profile| {
                profile.networks.update_with(NetworkID::Mainnet, |network| {
                    network.authorized_dapps = authorized_dapps.clone();
                });
                Ok(())
            })
        })
        .await
        .unwrap();

        os.mark_account_as_tombstoned(account.address)
            .await
            .unwrap();

        // ASSERT
        assert!(os
            .account_by_address(account.address)
            .unwrap()
            .is_tombstoned());

        let expected_authorized_dapps: AuthorizedDapps = serde_json::from_str(r#"
			[
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
				"displayName": "Radix Dashboard",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 2
							},
							"ids": [
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					},
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 2
							},
							"ids": [
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					}
				]
			},
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				"displayName": "Gumball Club",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 2
							},
							"ids": [
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
							]
						},
						"sharedPersonaData": {}
					}
				]
			}
        ]
            "#
        ).unwrap();

        let updated_authorized_dapps = os
            .profile()
            .unwrap()
            .clone()
            .current_network()
            .unwrap()
            .authorized_dapps
            .clone();
        pretty_assertions::assert_eq!(
            updated_authorized_dapps,
            expected_authorized_dapps
        )
    }

    #[actix_rt::test]
    async fn test_mark_accounts_as_tombstoned_become_tombstoned() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let account = Account::sample_mainnet();
        let other_account = Account::sample_mainnet_other();

        let authorized_dapps: AuthorizedDapps = serde_json::from_str(r#"
        [
			{
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
				"displayName": "Radix Dashboard",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 3
							},
							"ids": [
								"account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7",
                                "account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv"
							]
						},
						"sharedPersonaData": {}
					},
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 3
							},
							"ids": [
                                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7",
                                "account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv"
							]
						},
						"sharedPersonaData": {}
					}
				]
			},
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				"displayName": "Gumball Club",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 3
							},
							"ids": [
                                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7",
                                "account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv"
							]
						},
						"sharedPersonaData": {}
					}
				]
			}
        ]
            "#
        ).unwrap();

        // ACT
        os.with_timeout(|x| {
            join_all(vec![
                x.add_account(account.clone()),
                x.add_account(other_account.clone()),
                x.add_account(Account::sample_mainnet_carol()),
            ])
        })
        .await;

        os.with_timeout(|os| {
            os.update_profile_with(|profile| {
                profile.networks.update_with(NetworkID::Mainnet, |network| {
                    network.authorized_dapps = authorized_dapps.clone();
                });
                Ok(())
            })
        })
        .await
        .unwrap();

        os.with_timeout(|x| {
            x.mark_accounts_as_tombstoned(vec![
                account.address,
                other_account.address,
            ])
        })
        .await
        .unwrap();

        // ASSERT
        assert!(!os
            .account_by_address(Account::sample_mainnet_carol().address)
            .unwrap()
            .is_tombstoned());
        assert!([account.address, other_account.address].iter().all(
            |address| os.account_by_address(*address).unwrap().is_tombstoned()
        ));

        let expected_authorized_dapps: AuthorizedDapps = serde_json::from_str(r#"
			[
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
				"displayName": "Radix Dashboard",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "exactly",
								"quantity": 3
							},
							"ids": [
								"account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv"
							]
						},
						"sharedPersonaData": {}
					},
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 3
							},
							"ids": [
								"account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv"
							]
						},
						"sharedPersonaData": {}
					}
				]
			},
            {
				"networkID": 1,
				"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
				"displayName": "Gumball Club",
				"referencesToAuthorizedPersonas": [
					{
						"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"lastLogin": "2024-01-31T14:23:45.000Z",
						"sharedAccounts": {
							"request": {
								"quantifier": "atLeast",
								"quantity": 3
							},
							"ids": [
								"account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv"
							]
						},
						"sharedPersonaData": {}
					}
				]
			}
        ]
            "#
        ).unwrap();

        let updated_authorized_dapps = os
            .profile()
            .unwrap()
            .clone()
            .current_network()
            .unwrap()
            .authorized_dapps
            .clone();
        pretty_assertions::assert_eq!(
            updated_authorized_dapps,
            expected_authorized_dapps
        )
    }
}

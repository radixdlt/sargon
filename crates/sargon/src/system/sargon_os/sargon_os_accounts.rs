use std::{borrow::Borrow, sync::RwLockWriteGuard};

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
        .map(|(x, _)| x)
    }

    pub async fn create_unsaved_account_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let key_derivation_interactors = self.keys_derivation_interactors();

        let profile = self.profile()?;

        let (factor_source_id, account, derivation_outcome) = profile
            .create_unsaved_account_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                name,
                &self.clients.factor_instances_cache,
                key_derivation_interactors,
            )
            .await?;

        // TODO: move this to the FactorInstancesProvider... it should take a `emit_last_used` closure
        // Change of `last_used_on` of FactorSource
        self.update_last_used_of_factor_source(factor_source_id)
            .await?;

        Ok((account, derivation_outcome))
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
        let (account, derivation_outcome) = self
            .create_unsaved_account_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                name,
            )
            .await?;
        debug!("Created account, now saving it to profile.");
        self.add_account(account.clone()).await?;
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
    ) -> Result<()> {
        self.batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(count, network_id, name_prefix).await.map(|_|{})
    }

    pub async fn batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<FactorInstancesProviderOutcomeForFactor> {
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
    pub async fn batch_create_many_accounts_with_factor_source_then_save_once(
        &self,
        factor_source: FactorSource,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<()> {
        self.batch_create_many_accounts_with_factor_source_with_derivation_outcome_then_save_once(factor_source, count, network_id, name_prefix).await.map(|_|{})
    }
    pub async fn batch_create_many_accounts_with_factor_source_with_derivation_outcome_then_save_once(
        &self,
        factor_source: FactorSource,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<FactorInstancesProviderOutcomeForFactor> {
        debug!("Batch creating #{} accounts.", count);
        let (accounts, derivation_outcome) = self
            .batch_create_unsaved_accounts_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                count,
                name_prefix,
            )
            .await?;
        debug!("Created #{} accounts, now saving them to profile.", count);
        self.add_accounts(accounts).await?;
        info!(
            "Created account and saved #{} new accounts into profile",
            count
        );
        Ok(derivation_outcome)
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
    ) -> Result<Accounts> {
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
    ) -> Result<Accounts> {
        self.batch_create_unsaved_accounts_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            count,
            name_prefix,
        )
        .await
        .map(|(x, _)| x)
    }
    pub async fn batch_create_unsaved_accounts_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<(Accounts, FactorInstancesProviderOutcomeForFactor)> {
        let key_derivation_interactors = self.keys_derivation_interactors();

        let profile = self.profile()?;

        let (factor_source_id, accounts, derivation_outcome) = profile
            .create_unsaved_accounts_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                count,
                &self.clients.factor_instances_cache,
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

        Ok((accounts, derivation_outcome))
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
        let address = account.address;

        debug!("Adding account address: {} to profile", address);

        self.add_accounts_without_emitting_account_added_event(Accounts::just(
            account,
        ))
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::AccountAdded { address },
            ))
            .await;

        Ok(())
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
        let addresses = accounts
            .clone()
            .into_iter()
            .map(|a| a.address)
            .collect_vec();

        self.add_accounts_without_emitting_account_added_event(accounts)
            .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::AccountsAdded { addresses },
            ))
            .await;

        Ok(())
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
        self.update_profile_with(|p| {
            if p.update_account(&updated.address, |old| *old = updated.clone())
                .is_none()
            {
                Err(CommonError::UnknownAccount)
            } else {
                Ok(())
            }
        })
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::AccountUpdated {
                    address: updated.address,
                },
            ))
            .await;

        Ok(())
    }
}

impl SargonOS {
    /// Adds the `accounts` to active profile and **saves** the updated profile to
    /// secure storage, without emitting `Event::AccountAdded`, but we DO emit
    /// `Event::ProfileSaved`.`
    ///
    /// Returns `Ok(())` if the `accounts` were new and successfully added. If
    /// saving failed or if the accounts were already present in Profile, an
    /// error is returned.
    ///
    /// # Emits
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    async fn add_accounts_without_emitting_account_added_event(
        &self,
        accounts: Accounts,
    ) -> Result<()> {
        if accounts.is_empty() {
            warn!("Tried to add empty accounts...");
            return Ok(());
        }

        let number_of_accounts_to_add = accounts.len();

        let network_id = accounts
            .assert_elements_on_same_network()?
            .expect("Should have handled empty accounts case already.");

        debug!("Adding #{} accounts to Profile Network with ID: {} - or creating a Profile Network if it does not exist", number_of_accounts_to_add, network_id);

        self.update_profile_with(|p| {
            let networks = &mut p.networks;

            if networks.contains_id(network_id) {
                debug!("Profile already contained network to add #{} account(s) to, network_id: {}", number_of_accounts_to_add, network_id);
                networks
                    .try_try_update_with(&network_id, |network| {
                        let count_before = network.accounts.len();
                        debug!("Profile Network to add #{} account(s) to contains #{} accounts (before adding).", number_of_accounts_to_add, count_before);
                        network.accounts.extend(accounts.clone());
                        let count_after = network.accounts.len();
                        debug!("Profile Network now contains: #{} accounts", count_after);
                        if network.accounts.len() == count_before + number_of_accounts_to_add {
                            Ok(())
                        } else {
                            Err(CommonError::UnableToAddAllAccountsDuplicatesFound)
                        }
                    })
            } else {
                debug!("No Profile Network exists with ID {}, creating it...", network_id);
                let network = ProfileNetwork::new(
                    network_id,
                    accounts.clone(),
                    Personas::default(),
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

// # Securify
impl SargonOS {
    pub(crate) async fn securify_entities<E: IsSecurifiedEntity>(
        &mut self,
        addresses_of_entities: IndexSet<
            <E::BaseEntity as IsBaseEntity>::Address,
        >,
        shield: MatrixOfFactorSources,
    ) -> Result<(IndexSet<E>, FactorInstancesProviderOutcome)>
    where
        E::BaseEntity: IsEntity,
    {
        let profile_snapshot = self.profile()?;
        let key_derivation_interactors = self.keys_derivation_interactors();

        let outcome = SecurifyEntityFactorInstancesProvider::for_entity_mfa::<
            E::BaseEntity,
        >(
            &self.clients.factor_instances_cache,
            &profile_snapshot,
            shield.clone(),
            addresses_of_entities.clone(),
            key_derivation_interactors,
        )
        .await?;

        let instance_per_factor = outcome
            .clone()
            .per_factor
            .into_iter()
            .map(|(k, outcome_per_factor)| {
                (k, outcome_per_factor.to_use_directly)
            })
            .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>();

        assert_eq!(
            instance_per_factor
                .keys()
                .cloned()
                .collect::<HashSet<FactorSourceIDFromHash>>(),
            shield
                .all_factors()
                .into_iter()
                .map(|f| f.id_from_hash())
                .collect::<HashSet<FactorSourceIDFromHash>>()
        );

        // Now we need to map the flat set of instances into many MatrixOfFactorInstances, and assign
        // one to each account
        let updated_entities = addresses_of_entities
            .clone()
            .into_iter()
            .map(|a| {
                let entity =
                    profile_snapshot.get_entity::<E::BaseEntity>(&a).unwrap();
                // let matrix_of_instances =
                //     MatrixOfFactorInstances::fulfilling_matrix_of_factor_sources_with_instances(
                //         &mut instance_per_factor,
                //         shield.clone(),
                //     )
                //     .unwrap();

                // let access_controller = match entity.security_state() {
                //     EntitySecurityState::Unsecured(_) => {
                //         AccessController::from_unsecurified_address(a)
                //     }
                //     EntitySecurityState::Securified(sec) => sec.access_controller.clone(),
                // };
                // let veci = match entity.security_state() {
                //     EntitySecurityState::Unsecured(veci) => Some(veci),
                //     EntitySecurityState::Securified(sec) => {
                //         sec.veci.clone().map(|x| x.factor_instance())
                //     }
                // };
                // let sec = SecuredEntityControl::new(
                //     matrix_of_instances,
                //     access_controller,
                //     veci.map(|x| VirtualEntityCreatingInstance::new(x, entity.address())),
                // );

                // E::new(entity.name(), entity.entity_address(), sec)
                todo!()
            })
            .collect::<IndexSet<E>>();

        todo!()

        // for entity in updated_entities.clone().into_iter() {
        //     self.profile
        //         .try_write()
        //         .unwrap()
        //         .update_entity::<E::BaseEntity>(entity.into())
        // }
        // assert!(
        //     instance_per_factor.values().all(|x| x.is_empty()),
        //     "should have used all instances, but have unused instances: {:?}",
        //     instance_per_factor
        // );

        // Ok((updated_entities, outcome))
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
    #[ignore]
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
    #[ignore]
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
                x.create_unsaved_unnamed_mainnet_account_with_bdfs()
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(first, second);
    }

    #[actix_rt::test]
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
    async fn test_create_unsaved_account_emits_factor_source_updated() {
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
    #[ignore]
    async fn test_create_and_save_new_account_emits_events() {
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

    impl DisplayName {
        fn random() -> Self {
            Self::new(format!(
                "random-{}",
                id().to_string().drain(0..20).collect::<String>()
            ))
            .unwrap()
        }
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
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

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
    async fn test_accounts_on_current_network_empty() {
        let os = SUT::fast_boot().await;
        assert_eq!(os.accounts_on_current_network().unwrap(), Accounts::new());
    }

    #[actix_rt::test]
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
}

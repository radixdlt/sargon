use std::borrow::Borrow;

use crate::prelude::*;

/// If we wanna create an Olympia DeviceFactorSource or
/// a Babylon one, either main or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceFactorSourceType {
    Babylon { is_main: bool },
    Olympia,
}

impl SargonOS {
    /// Returns the "main Babylon" `DeviceFactorSource` of the current account as
    /// a `DeviceFactorSource`.
    pub fn main_bdfs(&self) -> Result<DeviceFactorSource> {
        self.profile_state_holder
            .access_profile_with(|p| p.main_bdfs())
    }

    /// Returns all the factor sources
    pub fn factor_sources(&self) -> Result<FactorSources> {
        self.profile_state_holder
            .access_profile_with(|p| p.factor_sources.clone())
    }

    /// Updates the factor source `updated` by mutating current profile and persisting
    /// the change to secure storage. Throws `UpdateFactorSourceMutateFailed` error if the
    /// factor source is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn update_factor_source(
        &self,
        updated: FactorSource,
    ) -> Result<()> {
        let id = updated.factor_source_id();

        debug!("Updating FactorSource with ID: {}", &id);

        self.update_profile_with(|p| {
            // p.update_last_used_of_factor_source(&id)
            p.update_any_factor_source(&id, |fs| *fs = updated.clone())
                .map_err(|_| CommonError::UpdateFactorSourceMutateFailed)
        })
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceUpdated { id },
            ))
            .await;

        Ok(())
    }

    /// Returns `Ok(false)` if the Profile already contained a factor source with the
    /// same id (Profile unchanged occurred).
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceAdded }`
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`,
    /// if the newly added FactorSource is a new **main** flag, then we remove the
    /// main flag from the old BDFS.
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn add_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        let id = factor_source.factor_source_id();

        let contains = self.profile_contains_factor_source(id).await?;

        if contains {
            return Ok(false);
        }

        self.add_factor_sources_without_emitting_factor_sources_added(
            FactorSources::just(factor_source),
        )
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceAdded { id },
            ))
            .await;

        Ok(true)
    }

    /// Adds all of the provided `factor_sources` to Profile in one single go.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourcesAdded }`
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`,
    /// if the newly added FactorSource is a new **main** flag, then we remove the
    /// main flag from the old BDFS.
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn add_factor_sources(
        &self,
        factor_sources: FactorSources,
    ) -> Result<Vec<FactorSourceID>> {
        let ids = self
            .add_factor_sources_without_emitting_factor_sources_added(
                factor_sources,
            )
            .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourcesAdded { ids: ids.clone() },
            ))
            .await;

        Ok(ids)
    }

    /// Updates the name of the corresponding `factor_source` in Profile. Throws `UpdateFactorSourceMutateFailed` error if the
    /// factor source is not found. Returns the updated `FactorSource`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn update_factor_source_name(
        &self,
        factor_source: FactorSource,
        name: String,
    ) -> Result<FactorSource> {
        let mut factor_source = factor_source;
        factor_source.set_name(name);
        self.update_factor_source(factor_source.clone()).await?;
        Ok(factor_source)
    }

    pub async fn debug_add_all_sample_factor_sources(
        &self,
    ) -> Result<Vec<FactorSourceID>> {
        self.debug_add_all_sample_factors_with_filter(|_| true)
            .await
    }

    pub async fn debug_add_all_sample_hd_factor_sources(
        &self,
    ) -> Result<Vec<FactorSourceID>> {
        self.debug_add_all_sample_factors_with_filter(|f| {
            f.factor_source_id().is_hash()
        })
        .await
    }

    pub async fn debug_add_all_sample_factors_with_filter(
        &self,
        filter: impl Fn(&FactorSource) -> bool,
    ) -> Result<Vec<FactorSourceID>> {
        let mwp = MnemonicWithPassphrase::sample_device();
        let id = FactorSourceIDFromHash::new_for_device(&mwp);
        self.clients
            .secure_storage
            .save_mnemonic_with_passphrase(&mwp, &id)
            .await?;

        let mwp = MnemonicWithPassphrase::sample_device_other();
        let id = FactorSourceIDFromHash::new_for_device(&mwp);
        self.clients
            .secure_storage
            .save_mnemonic_with_passphrase(&mwp, &id)
            .await?;

        let mwp = MnemonicWithPassphrase::sample_device_12_words();
        let id = FactorSourceIDFromHash::new_for_device(&mwp);
        self.clients
            .secure_storage
            .save_mnemonic_with_passphrase(&mwp, &id)
            .await?;

        self.add_factor_sources(
            FactorSources::sample_values_all()
                .into_iter()
                .filter(filter)
                .collect(),
        )
        .await
    }

    /// Creates a new unsaved DeviceFactorSource from the provided `mnemonic_with_passphrase`,
    /// either a "BDFS" or an "Olympia" one.
    pub async fn create_device_factor_source(
        &self,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        factor_type: DeviceFactorSourceType,
    ) -> Result<DeviceFactorSource> {
        let host_info = self.host_info().await;
        let factor_source = match factor_type {
            DeviceFactorSourceType::Olympia => DeviceFactorSource::olympia(
                &mnemonic_with_passphrase,
                &host_info,
            ),
            DeviceFactorSourceType::Babylon { is_main } => {
                DeviceFactorSource::babylon(
                    is_main,
                    &mnemonic_with_passphrase,
                    &host_info,
                )
            }
        };
        Ok(factor_source)
    }

    /// Loads a `MnemonicWithPassphrase` with the `id` of `device_factor_source`,
    /// from SecureStorage, and returns a `PrivateHierarchicalDeterministicFactorSource`
    /// built from both.
    ///
    /// Useful for when you will want to sign transactions or derive public keys for
    /// creation of new entities.
    ///
    /// Returns `Err` if loading or decoding of `MnemonicWithPassphrase` from
    /// SecureStorage fails.
    pub async fn load_private_device_factor_source_by_id(
        &self,
        id: impl Borrow<FactorSourceIDFromHash>,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        let id = id.borrow();
        let device_factor_source = self
            .profile_state_holder
            .try_access_profile_with(|p| p.device_factor_source_by_id(id))?;
        self.load_private_device_factor_source(&device_factor_source)
            .await
    }

    /// Accesses the active profile and checks if it contains a factor source with the given `id`.
    pub async fn profile_contains_factor_source(
        &self,
        id: FactorSourceID,
    ) -> Result<bool> {
        Ok(self.factor_source_ids()?.contains(&id))
    }
}

impl SargonOS {
    /// Adds all factor sources to Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`,
    /// if any of the newly added FactorSources has **main** flag, then we remove the
    /// main flag from the old BDFS.
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    async fn add_factor_sources_without_emitting_factor_sources_added(
        &self,
        factor_sources: FactorSources,
    ) -> Result<Vec<FactorSourceID>> {
        let ids_of_factors_to_add = factor_sources
            .iter()
            .map(|x| x.id())
            .collect::<IndexSet<_>>();
        let existing_ids = self
            .factor_source_ids()?
            .into_iter()
            .collect::<IndexSet<_>>();

        let ids_of_new_factor_sources = ids_of_factors_to_add
            .difference(&existing_ids)
            .cloned()
            .collect::<IndexSet<_>>();

        let new_factors_only = factor_sources
            .iter()
            .filter(|x| {
                ids_of_new_factor_sources.contains(&x.factor_source_id())
            })
            .collect::<FactorSources>();

        let is_any_of_new_factors_main_bdfs =
            new_factors_only.iter().any(|x| x.is_main_bdfs());
        let id_of_old_bdfs = self.main_bdfs()?.factor_source_id();

        for factor_source in new_factors_only.iter() {
            if !factor_source.factor_source_id().is_hash() {
                continue;
            }
            // Use FactorInstancesProvider to eagerly fill cache...
            // only test for now, need to do more integration work in hosts before enabling this
            #[cfg(debug_assertions)]
            let _ = self
                .pre_derive_and_fill_cache_with_instances_for_factor_source(
                    factor_source,
                    NetworkID::Mainnet, // we care not about other networks here
                )
                .await?;
        }

        self.update_profile_with(|p| {
            p.factor_sources.extend(new_factors_only.clone());
            Ok(())
        })
        .await?;

        if is_any_of_new_factors_main_bdfs {
            self.update_factor_source_remove_flag_main(id_of_old_bdfs)
                .await?;
            assert!(ids_of_factors_to_add
                .contains(&self.main_bdfs()?.factor_source_id()))
        }

        Ok(ids_of_new_factor_sources.into_iter().collect_vec())
    }

    /// Returns IDs of all the factor sources.
    pub fn factor_source_ids(&self) -> Result<HashSet<FactorSourceID>> {
        self.profile_state_holder.access_profile_with(|p| {
            HashSet::from_iter(p.factor_sources.iter().map(|s| s.id()))
        })
    }

    /// Updates the `last_used_on` for the factor source and emits events.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn update_last_used_of_factor_source(
        &self,
        factor_source_id: impl Into<FactorSourceID>,
    ) -> Result<()> {
        let id = factor_source_id.into();

        debug!(
            "Updating 'last_used_on' date for FactorSource with ID: {}",
            &id
        );

        self.update_profile_with(|p| p.update_last_used_of_factor_source(&id))
            .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceUpdated { id },
            ))
            .await;

        Ok(())
    }

    pub async fn update_factor_source_remove_flag_main(
        &self,
        factor_source_id: impl Into<FactorSourceID>,
    ) -> Result<()> {
        let id = factor_source_id.into();

        debug!(
            "Updating 'flags', removing main, for FactorSource with ID: {}",
            &id
        );

        self.update_profile_with(|p| {
            p.update_factor_source_remove_flag_main(&id)
        })
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceUpdated { id },
            ))
            .await;

        Ok(())
    }

    /// Set the FactorSource with the given `factor_source_id` as the main factor source of its kind.
    /// Throws `UpdateFactorSourceMutateFailed` error if the factor source is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    ///
    /// If there is any main `FactorSource` of the given `FactorSourceKind`, such events are emitted also when
    /// removing the flag from the old main factor source.
    pub async fn set_main_factor_source(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<()> {
        // Get current main factor source ID (if any)
        let current_main_id =
            self.profile_state_holder.access_profile_with(|p| {
                p.main_factor_source_of_kind(
                    factor_source_id.get_factor_source_kind(),
                )
            })?;

        let updated_ids = match current_main_id {
            Some(current_main_id) => vec![current_main_id, factor_source_id],
            None => vec![factor_source_id],
        };

        self.update_profile_with(|p| {
            // Remove main flag from current main (if any)
            if let Some(current_main_id) = current_main_id {
                p.update_factor_source_remove_flag_main(&current_main_id)
                    .map_err(|_| CommonError::UpdateFactorSourceMutateFailed)?;
            }

            // Add the flag to the new main factor source
            p.update_factor_source_add_flag_main(&factor_source_id)
                .map_err(|_| CommonError::UpdateFactorSourceMutateFailed)
        })
        .await?;

        // Emit event
        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourcesUpdated { ids: updated_ids },
            ))
            .await;

        Ok(())
    }
}

impl SargonOS {
    /// Tries to load a `MnemonicWithPassphrase` from secure storage
    /// by `id` of type `FactorSourceIDFromHash`.
    pub async fn mnemonic_with_passphrase_of_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.secure_storage.load_mnemonic_with_passphrase(id).await
    }

    /// Loads a `MnemonicWithPassphrase` with the `id` of `device_factor_source`,
    /// from SecureStorage, and returns a `PrivateHierarchicalDeterministicFactorSource`
    /// built from both.
    ///
    /// Useful for when you will want to sign transactions or derive public keys for
    /// creation of new entities.
    ///
    /// Returns `Err` if loading or decoding of `MnemonicWithPassphrase` from
    /// SecureStorage fails.
    pub async fn load_private_device_factor_source(
        &self,
        device_factor_source: &DeviceFactorSource,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        info!(
            "Load Private DeviceFactorSource from SecureStorage, factor source id: {}",
            &device_factor_source.id
        );
        self.mnemonic_with_passphrase_of_device_factor_source_by_id(
            &device_factor_source.id,
        )
        .await
        .map(|mwp| {
            PrivateHierarchicalDeterministicFactorSource::new(
                mwp,
                device_factor_source.clone(),
            )
        })
        // .log_info(
        //     "Successfully loaded Private DeviceFactorSource from SecureStorage",
        // )
    }

    /// Tries to load a `MnemonicWithPassphrase` from secure storage
    /// by `factor_source_id`.
    pub async fn mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> Result<MnemonicWithPassphrase> {
        let id = (*factor_source_id)
            .into_hash()
            .map_err(|_| CommonError::FactorSourceIDNotFromHash)?;
        self.mnemonic_with_passphrase_of_device_factor_source_by_id(&id)
            // tarpaulin will incorrectly flag next line is missed
            .await
    }

    pub async fn factor_instances_in_cache(
        &self,
    ) -> IndexMap<
        FactorSourceIDFromHash,
        Vec<IndexSet<HierarchicalDeterministicFactorInstance>>,
    > {
        let cache = self.cache_snapshot().await;
        let cache = cache.serializable_snapshot();
        cache
            .0
            .into_iter()
            .map(|(k, v)| {
                let fsid = FactorSourceIDFromHash::from(k);
                let vec_of_sets: Vec<
                    IndexSet<HierarchicalDeterministicFactorInstance>,
                > = v
                    .into_iter()
                    .map(|(_, x)| {
                        x.into_iter()
                            .map(|y| {
                                HierarchicalDeterministicFactorInstance::new(
                                    fsid, y,
                                )
                            })
                            .collect::<IndexSet<_>>()
                    })
                    .collect_vec();

                (fsid, vec_of_sets)
            })
            .collect::<IndexMap<
                FactorSourceIDFromHash,
                Vec<IndexSet<HierarchicalDeterministicFactorInstance>>,
            >>()
    }

    pub async fn cache_snapshot(&self) -> FactorInstancesCache {
        self.clients
            .factor_instances_cache
            .snapshot()
            .await
            .unwrap()
    }
}

impl SargonOS {
    /// Triggers the spot check for the given factor source, and returns whether the spot check was successful.
    pub async fn trigger_spot_check(
        &self,
        factor_source: FactorSource,
        allow_skip: bool,
    ) -> Result<bool> {
        let response = self
            .spot_check_interactor()
            .spot_check(factor_source, allow_skip)
            .await?;
        match response {
            SpotCheckResponse::Valid => Ok(true),
            SpotCheckResponse::Skipped => Ok(false),
        }
    }

    /// If necessary, performs a spot check on the factor source before creating an entity.
    ///
    /// We only perform the spot check when we have enough keys in the cache to create the entity without deriving anymore.
    pub async fn spot_check_factor_source_before_entity_creation_if_necessary(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        entity_kind: EntityKind,
    ) -> Result<()> {
        let cache = self.cache_snapshot().await;
        let should_spot_check = cache.is_entity_creation_satisfied(
            network_id,
            factor_source.id_from_hash(),
            entity_kind,
        );
        if should_spot_check {
            debug!("Spot checking the factor source...");
            let response = self
                .spot_check_interactor()
                .spot_check(factor_source, false)
                .await?;
            debug!("Spot check response: {:?}", response);
        } else {
            debug!("No need to spot check the factor source.");
        }
        Ok(())
    }
}

#[cfg(debug_assertions)]
impl SargonOS {
    /// For tests
    pub async fn clear_cache(&self) {
        println!("💣 CLEAR CACHE");
        self.clients.factor_instances_cache.clear().await.unwrap();
    }

    /// For tests
    pub async fn set_cache(
        &self,
        cache_snapshot: FactorInstancesCacheSnapshot,
    ) {
        self.clients
            .factor_instances_cache
            .set_cache(cache_snapshot)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_bdfs() {
        // ARRANGE
        let mwp = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        // ACT
        let loaded = os.main_bdfs().unwrap();

        // ASSERT
        assert_eq!(
            loaded.factor_source_id(),
            FactorSourceIDFromHash::new_for_device(&mwp).into()
        );
        assert_eq!(
            os.factor_sources().unwrap(),
            FactorSources::just(FactorSource::from(loaded))
        );
    }

    #[actix_rt::test]
    async fn test_add_debug_factor_sources() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let added = os.debug_add_all_sample_factor_sources().await.unwrap();

        // ASSERT
        assert!(os
            .factor_sources()
            .unwrap()
            .into_iter()
            .map(|f| f.id())
            .collect::<HashSet<_>>()
            .is_superset(&added.into_iter().collect::<HashSet<_>>()));
    }

    #[actix_rt::test]
    async fn test_load_private_device_factor_source_by_id() {
        // ARRANGE
        let mwp = MnemonicWithPassphrase::sample();
        let factor_source_id = FactorSourceIDFromHash::new_for_device(&mwp);
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        // ACT
        let private = os
            .with_timeout(|x| {
                x.load_private_device_factor_source_by_id(&factor_source_id)
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(private.mnemonic_with_passphrase, mwp);
    }

    #[actix_rt::test]
    async fn test_mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(
    ) {
        // ARRANGE
        let mwp = MnemonicWithPassphrase::sample();
        let factor_source_id = FactorSourceIDFromHash::new_for_device(&mwp);
        let id = FactorSourceID::from(factor_source_id);
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        // ACT
        let loaded = os
      .with_timeout(|x| {
          x.mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(&id)
      })
      .await
      .unwrap();

        // ASSERT
        assert_eq!(loaded, mwp);
    }

    #[actix_rt::test]
    async fn test_add_ledger_factor_source() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let inserted = os
            .with_timeout(|x| x.add_factor_source(FactorSource::sample_other()))
            .await
            .unwrap();

        // ASSERT
        assert!(inserted);
        assert!(os
            .profile()
            .unwrap()
            .factor_sources
            .contains_by_id(&FactorSource::sample_other()));
    }

    #[actix_rt::test]
    async fn test_add_ledger_factor_source_new_bdfs_removes_main_from_existing_bdfs(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let old_bdfs_id = os.main_bdfs().unwrap().factor_source_id();
        let new_bdfs = DeviceFactorSource::babylon(
            true,
            &MnemonicWithPassphrase::sample(),
            &HostInfo::sample(),
        );
        assert_ne!(old_bdfs_id, new_bdfs.factor_source_id());

        // ACT
        let inserted = os
            .with_timeout(|x| {
                x.add_factor_source(FactorSource::from(new_bdfs.clone()))
            })
            .await
            .unwrap();

        // ASSERT
        assert!(inserted);
        assert_eq!(os.main_bdfs().unwrap(), new_bdfs);
        let old_bdfs = os
            .profile()
            .unwrap()
            .factor_sources
            .get_id(old_bdfs_id)
            .unwrap()
            .clone()
            .into_device()
            .unwrap();
        assert!(!old_bdfs.is_main_bdfs());
    }

    #[actix_rt::test]
    async fn test_add_existing_factor_source_is_noop() {
        // ARRANGE
        let mwp = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        let bdfs = os.main_bdfs().unwrap();

        // ACT
        let inserted = os
            .with_timeout(|x| {
                x.add_factor_source(
                    DeviceFactorSource::babylon(
                        false,
                        &mwp,
                        &HostInfo::sample_other(),
                    )
                    .into(),
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert!(!inserted); // already exists
        assert_eq!(
            os.profile().unwrap().factor_sources,
            FactorSources::just(bdfs.into())
        );
    }

    #[actix_rt::test]
    async fn test_create_device_factor_source_babylon_main() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let bdfs = os
            .with_timeout(|x| {
                x.create_device_factor_source(
                    MnemonicWithPassphrase::sample(),
                    DeviceFactorSourceType::Babylon { is_main: true },
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert!(bdfs.is_main_bdfs());
    }

    #[actix_rt::test]
    async fn test_create_device_factor_source_babylon_not_main() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let bdfs = os
            .with_timeout(|x| {
                x.create_device_factor_source(
                    MnemonicWithPassphrase::sample(),
                    DeviceFactorSourceType::Babylon { is_main: false },
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert!(!bdfs.is_main_bdfs());
        assert!(!bdfs.common.is_main());
        assert!(bdfs.common.supports_babylon());
    }

    #[actix_rt::test]
    async fn test_create_device_factor_source_olympia() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let dfs = os
            .with_timeout(|x| {
                x.create_device_factor_source(
                    MnemonicWithPassphrase::sample_device_12_words(),
                    DeviceFactorSourceType::Olympia,
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert!(!dfs.common.is_main());
        assert!(!dfs.common.supports_babylon());
        assert!(dfs.common.supports_olympia());
        assert_eq!(
            dfs.factor_source_id(),
            DeviceFactorSource::sample_other().factor_source_id()
        );
    }

    #[actix_rt::test]
    async fn when_adding_many_factor_sources_event_factor_sources_added_is_emitted(
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
        os.with_timeout(|x| x.new_wallet(false)).await.unwrap();

        // ACT
        let ids = os
            .with_timeout(|x| {
                x.add_factor_sources(FactorSources::sample_values_all_hd())
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            ids.clone(),
            FactorSources::sample_values_all_hd()
                .into_iter()
                .map(|x| x.id())
                .collect_vec(),
        );
        assert!(event_bus_driver.recorded().iter().any(|e| e.event
            == Event::ProfileModified {
                change: EventProfileModified::FactorSourcesAdded {
                    ids: ids.clone()
                }
            }));
    }

    #[actix_rt::test]
    async fn test_debug_add_all_sample_factors_saves_mnemonics_to_secure_storage(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();

        // ASSERT
        // First
        let expected = MnemonicWithPassphrase::sample_device();
        let id = FactorSourceIDFromHash::new_for_device(&expected);
        let loaded = os
            .with_timeout(|x| {
                x.secure_storage.load_mnemonic_with_passphrase(&id)
            })
            .await
            .unwrap();
        assert_eq!(loaded, expected);

        // Second
        let expected = MnemonicWithPassphrase::sample_device_other();
        let id = FactorSourceIDFromHash::new_for_device(&expected);
        let loaded = os
            .with_timeout(|x| {
                x.secure_storage.load_mnemonic_with_passphrase(&id)
            })
            .await
            .unwrap();
        assert_eq!(loaded, expected);

        // Third
        let expected = MnemonicWithPassphrase::sample_device_12_words();
        let id = FactorSourceIDFromHash::new_for_device(&expected);
        let loaded = os
            .with_timeout(|x| {
                x.secure_storage.load_mnemonic_with_passphrase(&id)
            })
            .await
            .unwrap();
        assert_eq!(loaded, expected);
    }

    #[actix_rt::test]
    async fn test_update_factor_source() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let mut factor = ArculusCardFactorSource::sample();
        os.with_timeout(|x| x.add_factor_source(factor.clone().into()))
            .await
            .unwrap();

        // ACT
        let new_label = "new important name";
        factor.hint.label = new_label.to_owned();
        os.with_timeout(|x| x.update_factor_source(factor.clone().into()))
            .await
            .unwrap();

        // ASSERT
        assert!(os.profile().unwrap().factor_sources.into_iter().any(|f| {
            match f {
                FactorSource::ArculusCard { value } => {
                    value.hint.label == *new_label
                }
                _ => false,
            }
        }));
    }

    #[actix_rt::test]
    async fn test_update_factor_source_name() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let factor = ArculusCardFactorSource::sample();
        os.with_timeout(|x| x.add_factor_source(factor.clone().into()))
            .await
            .unwrap();

        // ACT
        let new_name = "My updated name";
        os.with_timeout(|x| {
            x.update_factor_source_name(
                factor.clone().into(),
                new_name.to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        assert!(os.profile().unwrap().factor_sources.into_iter().any(|f| {
            match f {
                FactorSource::ArculusCard { value } => {
                    value.name() == *new_name
                }
                _ => false,
            }
        }));
    }

    #[actix_rt::test]
    async fn set_main_factor_source_fails_if_factor_source_not_added() {
        let os = SUT::fast_boot().await;

        let error = os
            .with_timeout(|x| {
                x.set_main_factor_source(FactorSource::sample_password().id())
            })
            .await
            .expect_err("Expected an error");

        assert_eq!(error, CommonError::UpdateFactorSourceMutateFailed);
    }

    #[actix_rt::test]
    async fn set_main_factor_source_with_previous_main() {
        // Set up OS with `previous_main` as the main device factor source
        let event_bus_driver = RustEventBusDriver::new();
        let os = boot(event_bus_driver.clone()).await;

        let previous_main = DeviceFactorSource::sample();
        let new_main = DeviceFactorSource::sample_other();
        let factor_sources = FactorSources::from_iter([
            previous_main.clone().into(),
            new_main.clone().into(),
        ]);

        os.with_timeout(|x| {
            x.add_factor_sources_without_emitting_factor_sources_added(
                factor_sources.clone(),
            )
        })
        .await
        .unwrap();

        // Verify that `previous_main` is the main device factor source
        assert!(os
            .profile()
            .unwrap()
            .device_factor_source_by_id(&previous_main.id)
            .unwrap()
            .is_main_bdfs());

        // Clear recorded events
        event_bus_driver.clear_recorded();

        // Set `new_main` as main
        os.with_timeout(|x| {
            x.set_main_factor_source(new_main.clone().factor_source_id())
        })
        .await
        .unwrap();

        // Verify previous is no longer main, while new is
        let profile = os.profile().unwrap();
        assert!(!profile
            .device_factor_source_by_id(&previous_main.id)
            .unwrap()
            .common
            .is_main());
        assert!(profile
            .device_factor_source_by_id(&new_main.id)
            .unwrap()
            .common
            .is_main());

        // Verify 2 events are emitted: `ProfileSaved` and `FactorSourcesUpdated`
        let events = event_bus_driver.recorded();
        assert_eq!(events.len(), 2);
        assert!(events.iter().any(|e| e.event == Event::ProfileSaved),);
        assert!(events.iter().any(|e| e.event
            == Event::ProfileModified {
                change: EventProfileModified::FactorSourcesUpdated {
                    ids: vec![
                        previous_main.id.into(),
                        new_main.factor_source_id()
                    ],
                }
            }));
    }

    #[actix_rt::test]
    async fn set_main_factor_source_without_previous_main() {
        // Set up OS with no main arculus factor source
        let event_bus_driver = RustEventBusDriver::new();
        let os = boot(event_bus_driver.clone()).await;

        let factor_source = ArculusCardFactorSource::sample();
        os.with_timeout(|x| x.add_factor_source(factor_source.clone().into()))
            .await
            .unwrap();

        // Verify that there is no main arculus factor source
        assert!(!os
            .profile()
            .unwrap()
            .factor_sources
            .iter()
            .any(|f| f.is_arculus_card() && f.common_properties().is_main()));

        // Clear recorded events
        event_bus_driver.clear_recorded();

        // Set `factor_source` as main
        os.with_timeout(|x| {
            x.set_main_factor_source(factor_source.clone().factor_source_id())
        })
        .await
        .unwrap();

        // Verify we now have a main arculus factor source
        assert!(os
            .profile()
            .unwrap()
            .factor_sources
            .iter()
            .any(|f| f.is_arculus_card() && f.common_properties().is_main()));

        // Verify 2 events are emitted: `ProfileSaved` and `FactorSourceUpdated`
        let events = event_bus_driver.recorded();
        assert_eq!(events.len(), 2);
        assert!(events.iter().any(|e| e.event == Event::ProfileSaved));
        assert!(events.iter().any(|e| e.event
            == Event::ProfileModified {
                change: EventProfileModified::FactorSourcesUpdated {
                    ids: vec![factor_source.factor_source_id()],
                }
            }));
    }

    #[actix_rt::test]
    async fn trigger_spot_check_valid() {
        let clients = Clients::new(Bios::new(Drivers::test()));
        let interactors =
            Interactors::new_from_clients_and_spot_check_interactor(
                &clients,
                Arc::new(TestSpotCheckInteractor::new_succeeded()),
            );
        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();

        let result = os
            .with_timeout(|x| {
                x.trigger_spot_check(FactorSource::sample(), false)
            })
            .await
            .unwrap();

        assert!(result);
    }

    #[actix_rt::test]
    async fn trigger_spot_check_skipped() {
        let clients = Clients::new(Bios::new(Drivers::test()));
        let interactors =
            Interactors::new_from_clients_and_spot_check_interactor(
                &clients,
                Arc::new(TestSpotCheckInteractor::new_skipped()),
            );
        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();

        let result = os
            .with_timeout(|x| {
                x.trigger_spot_check(FactorSource::sample(), true)
            })
            .await
            .unwrap();

        assert!(!result);
    }

    #[actix_rt::test]
    async fn trigger_spot_check_failing() {
        let clients = Clients::new(Bios::new(Drivers::test()));
        let error = CommonError::sample();
        let interactors =
            Interactors::new_from_clients_and_spot_check_interactor(
                &clients,
                Arc::new(TestSpotCheckInteractor::new_failed(error.clone())),
            );
        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();

        let result = os
            .with_timeout(|x| {
                x.trigger_spot_check(FactorSource::sample(), false)
            })
            .await
            .expect_err("Expected an error");

        assert_eq!(result, error);
    }

    async fn boot(event_bus_driver: Arc<RustEventBusDriver>) -> Arc<SUT> {
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
        os
    }
}

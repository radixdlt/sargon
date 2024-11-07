use std::borrow::Borrow;

use crate::prelude::*;

#[derive(Debug)]
pub struct FactorInstancesCacheClient {
    conflict_resolution_strategy:
        FactorInstancesCacheConflictResolutionStrategy,
    file_system_client: Arc<FileSystemClient>,
    cache: RwLock<Option<FactorInstancesCache>>,
}

#[derive(Debug, Clone, Copy)]
enum FactorInstancesCacheConflictResolutionStrategy {
    #[allow(dead_code)]
    Panic,
    #[allow(dead_code)]
    UseLatestFromFileSystem,
    UseLatestInMemory {
        on_conflict_save_to_file: bool,
    },
}

impl FactorInstancesCacheClient {
    const CACHE_FILE: &'static str =
        "radix_babylon_wallet_pre_derived_public_keys_cache.json";
    fn with(
        conflict_resolution_strategy: FactorInstancesCacheConflictResolutionStrategy,
        file_system_client: Arc<FileSystemClient>,
    ) -> Self {
        Self {
            conflict_resolution_strategy,
            file_system_client,
            cache: RwLock::new(None),
        }
    }

    pub fn new(file_system_client: Arc<FileSystemClient>) -> Self {
        Self::with(
            FactorInstancesCacheConflictResolutionStrategy::UseLatestInMemory {
                on_conflict_save_to_file: true,
            },
            file_system_client,
        )
    }

    fn read_cache<R>(
        &self,
        access: impl FnOnce(&FactorInstancesCache) -> Result<R>,
    ) -> Result<R> {
        let guard = self.cache.read().unwrap();
        let cache = guard.as_ref().ok_or(CommonError::Unknown)?;
        access(cache)
    }
    fn update_cache<R>(
        &self,
        mut update: impl FnMut(&mut FactorInstancesCache) -> Result<R>,
    ) -> Result<R> {
        let mut guard = self.cache.write().unwrap();
        let cache = guard.as_mut().ok_or(CommonError::Unknown)?;
        update(cache)
    }

    async fn resolve_conflicts_if_needed(
        &self,
        in_memory: FactorInstancesCacheSnapshot,
    ) -> Result<()> {
        if let Some(on_file) = self.load_from_file().await.ok().flatten() {
            if on_file == in_memory {
                return Ok(());
            }
            match self.conflict_resolution_strategy {
                FactorInstancesCacheConflictResolutionStrategy::Panic => {
                    panic!("Conflict! Cache in memory differs from cache on file system.")
                }
                FactorInstancesCacheConflictResolutionStrategy::UseLatestFromFileSystem => {
                    warn!("Conflict! Cache in memory differs from cache on file system. Using cache from file system as specified by conflict_resolution_strategy.");
                    self.set_cache_to(on_file)
                }
                FactorInstancesCacheConflictResolutionStrategy::UseLatestInMemory {
                    on_conflict_save_to_file,
                } => {
                    if on_conflict_save_to_file {
                        warn!("Conflict! Cache in memory differs from cache on file system. Using cache from memory as specified by conflict_resolution_strategy and saving it to file system.");
                        self.save_to_file(in_memory).await
                    } else {
                        warn!("Conflict! Cache in memory differs from cache on file system. Using cache from memory as specified by conflict_resolution_strategy - but on_conflict_save_to_file was false, so will not save to file system right now.");
                        Ok(())
                    }
                }
            }
        } else {
            assert!(in_memory.is_empty());
            Ok(())
        }
    }

    pub async fn init_if_needed(&self) -> Result<()> {
        let maybe_in_memory: Option<FactorInstancesCacheSnapshot>;
        {
            maybe_in_memory = self
                .cache
                .read()
                .unwrap()
                .as_ref()
                .map(|c| c.serializable_snapshot());
        }
        if let Some(in_memory) = maybe_in_memory {
            self.resolve_conflicts_if_needed(in_memory).await
        } else {
            self.load().await
        }
    }

    async fn update_and_persist_cache<R>(
        &self,
        update: impl FnMut(&mut FactorInstancesCache) -> Result<R>,
    ) -> Result<R> {
        self.init_if_needed().await?;

        let out = self.update_cache(update)?;
        self.save().await?;
        Ok(out)
    }

    async fn access_cache_init_if_needed<R>(
        &self,
        access: impl FnOnce(&FactorInstancesCache) -> Result<R>,
    ) -> Result<R> {
        self.init_if_needed().await?;

        self.read_cache(access)
    }

    async fn load_from_file(
        &self,
    ) -> Result<Option<FactorInstancesCacheSnapshot>> {
        let maybe_json = self
            .file_system_client
            .load_from_file(Self::CACHE_FILE)
            .await?;

        let Some(json) = maybe_json else {
            return Ok(None);
        };

        let deserialized =
            json.deserialize::<FactorInstancesCacheSnapshot>()?;

        Ok(Some(deserialized))
    }

    fn set_cache_to(
        &self,
        cache_snapshot: FactorInstancesCacheSnapshot,
    ) -> Result<()> {
        let cache = FactorInstancesCache::from(cache_snapshot);
        let mut guard = self.cache.write().unwrap();
        guard.replace(cache);
        Ok(())
    }

    async fn load(&self) -> Result<()> {
        let maybe_cache_snapshot = self.load_from_file().await?;
        let cache_snapshot = maybe_cache_snapshot.unwrap_or_default();
        self.set_cache_to(cache_snapshot)
    }

    async fn save_to_file(
        &self,
        cache_snapshot: FactorInstancesCacheSnapshot,
    ) -> Result<()> {
        let json = cache_snapshot.serialize_to_bytes()?;

        self.file_system_client
            .save_to_file(Self::CACHE_FILE, &json)
            .await
    }
    async fn save(&self) -> Result<()> {
        let cache_snapshot: FactorInstancesCacheSnapshot;
        {
            let guard = self.cache.read().unwrap();
            let cache = guard.as_ref().expect("No cache in memory, this is a programmer error. Should have called 'init_if_needed' for all execution paths.");
            cache_snapshot = cache.serializable_snapshot();
        }
        self.save_to_file(cache_snapshot).await
    }
}

impl FactorInstancesCacheClient {
    pub async fn insert_for_factor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
        instances: &FactorInstances,
    ) -> Result<bool> {
        self.update_and_persist_cache(|cache| {
            cache.insert_for_factor(factor_source_id, instances)
        })
        .await
    }

    /// Inserts all instance in `per_factor`.
    pub async fn insert_all(
        &self,
        per_factor: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
    ) -> Result<()> {
        self.update_and_persist_cache(|cache| cache.insert_all(per_factor))
            .await
    }

    /// Returns the max derivation entity index for the given `factor_source_id` and `index_agnostic_path`.
    pub async fn max_index_for(
        &self,
        factor_source_id: impl Borrow<FactorSourceIDFromHash>,
        index_agnostic_path: impl Borrow<IndexAgnosticPath>,
    ) -> Result<Option<HDPathComponent>> {
        self.access_cache_init_if_needed(|cache| {
            Ok(cache.max_index_for(factor_source_id, index_agnostic_path))
        })
        .await
    }

    /// Returns enough instances to satisfy the requested quantity for each factor source,
    /// **OR LESS**, never more, and if less, it means we MUST derive more, and if we
    /// must derive more, this function returns the quantities to derive for each factor source,
    /// for each derivation preset, not only the originally requested one.
    pub async fn get_poly_factor_with_quantities(
        &self,
        factor_source_ids: &IndexSet<FactorSourceIDFromHash>,
        originally_requested_quantified_derivation_preset: &QuantifiedDerivationPreset,
        network_id: NetworkID,
    ) -> Result<CachedInstancesWithQuantitiesOutcome> {
        self.access_cache_init_if_needed(|cache| {
            cache.get_poly_factor_with_quantities(
                factor_source_ids,
                originally_requested_quantified_derivation_preset,
                network_id,
            )
        })
        .await
    }

    pub async fn delete(
        &self,
        instances_per_factor_sources_to_delete: &IndexMap<
            FactorSourceIDFromHash,
            FactorInstances,
        >,
    ) -> Result<()> {
        self.update_and_persist_cache(|cache| {
            cache.delete(instances_per_factor_sources_to_delete);
            Ok(())
        })
        .await
    }
}

#[cfg(test)]
impl FactorInstancesCacheClient {
    pub async fn insert_single(
        &self,
        instance: &HierarchicalDeterministicFactorInstance,
    ) -> Result<bool> {
        self.insert_for_factor(
            &instance.factor_source_id,
            &FactorInstances::from_iter([instance.clone()]),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorInstancesCacheClient;

    #[actix_rt::test]
    async fn test_load_modify_save_reload_with_strategy_latest_from_file_system(
    ) {
        let file_system = Arc::new(FileSystemClient::in_memory());

        // We MUST use `FactorInstancesCacheConflictResolutionStrategy::UseLatestFromFileSystem` for this
        // test to pass, since it will detect that the cache from file system is newer than in memory.
        let conflict_strategy = FactorInstancesCacheConflictResolutionStrategy::UseLatestFromFileSystem;
        let sut1 = SUT::with(conflict_strategy, file_system.clone());

        let fsid = FactorSourceIDFromHash::sample_at(0);

        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        );

        let one = Hardened::Unsecurified(
            UnsecurifiedHardened::try_from(1u32).unwrap(),
        );
        let fi1 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            one,
        );
        let one = HDPathComponent::from(one);
        let instances = FactorInstances::from_iter([fi0, fi1]);

        sut1.insert_for_factor(&fsid, &instances).await.unwrap();

        let max = sut1
            .max_index_for(
                fsid,
                DerivationPreset::AccountVeci
                    .index_agnostic_path_on_network(NetworkID::Mainnet),
            )
            .await
            .unwrap();
        assert_eq!(max, Some(one));

        // we might not need to set `conflict_strategy` here actually...
        let sut2 = SUT::with(conflict_strategy, file_system.clone());

        let max_sut2 = sut2
            .max_index_for(
                fsid,
                DerivationPreset::AccountVeci
                    .index_agnostic_path_on_network(NetworkID::Mainnet),
            )
            .await
            .unwrap();
        assert_eq!(max_sut2, Some(one));

        let five = Hardened::Unsecurified(
            UnsecurifiedHardened::try_from(5u32).unwrap(),
        );
        let fi5 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            five,
        );

        sut2.insert_single(&fi5).await.unwrap();

        let max_higher_sut1 = sut1
            .max_index_for(
                fsid,
                DerivationPreset::AccountVeci
                    .index_agnostic_path_on_network(NetworkID::Mainnet),
            )
            .await
            .unwrap();
        let five = HDPathComponent::from(five);
        assert_eq!(max_higher_sut1, Some(five));
    }

    #[actix_rt::test]
    async fn test_load_modify_save_reload_with_strategy_latest_from_memory_overwrite(
    ) {
        let file_system = Arc::new(FileSystemClient::in_memory());

        let sut1 = SUT::with( FactorInstancesCacheConflictResolutionStrategy::UseLatestFromFileSystem, file_system.clone());
        let sut2 = SUT::with(
            FactorInstancesCacheConflictResolutionStrategy::UseLatestInMemory {
                on_conflict_save_to_file: true,
            },
            file_system.clone(),
        );

        let fsid = FactorSourceIDFromHash::sample_at(0);

        let path = DerivationPreset::AccountVeci
            .index_agnostic_path_on_network(NetworkID::Mainnet);
        let max = sut1.max_index_for(fsid, path).await.unwrap();
        assert!(max.is_none());

        let max = sut2.max_index_for(fsid, path).await.unwrap();
        assert!(max.is_none());

        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(UnsecurifiedHardened::TWO),
        );

        sut1.insert_single(&fi0).await.unwrap();
        let max = sut1.max_index_for(fsid, path).await.unwrap();
        assert_eq!(
            max,
            Some(HDPathComponent::from(Hardened::Unsecurified(
                UnsecurifiedHardened::TWO
            )))
        );

        // Sut2 does not see it! Since Sut2 specified "UseLatestInMemory"
        let max = sut2.max_index_for(fsid, path).await.unwrap();
        assert!(max.is_none(),);

        // since we specified "on_conflict_save_to_file: true" on Sut2 AND Sut1 specifies
        // UseLatestFromFileSystem, Sut2 should have overriden the cache on file system =>
        // clearing sut1
        let max = sut1.max_index_for(fsid, path).await.unwrap();
        assert!(max.is_none()); // Overwritten!
    }

    #[actix_rt::test]
    #[should_panic]
    async fn test_load_modify_save_reload_with_strategy_panic() {
        let file_system = Arc::new(FileSystemClient::in_memory());

        let sut1 = SUT::with(
            FactorInstancesCacheConflictResolutionStrategy::Panic,
            file_system.clone(),
        );
        let sut2 = SUT::with(
            FactorInstancesCacheConflictResolutionStrategy::UseLatestInMemory {
                on_conflict_save_to_file: true,
            },
            file_system.clone(),
        );

        let fsid = FactorSourceIDFromHash::sample_at(0);

        let path = DerivationPreset::AccountVeci
            .index_agnostic_path_on_network(NetworkID::Mainnet);
        let max = sut1.max_index_for(fsid, path).await.unwrap();
        assert!(max.is_none()); // nothing written by any SUT yet

        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(UnsecurifiedHardened::TWO),
        );

        sut2.insert_single(&fi0).await.unwrap();

        // Should panic
        let _ = sut1.max_index_for(fsid, path).await.unwrap();
    }

    #[actix_rt::test]
    async fn test_load_modify_save_reload_with_strategy_should_not_panic_if_lazy_inits_after_other_sut_has_saved(
    ) {
        let file_system = Arc::new(FileSystemClient::in_memory());

        let sut1 = SUT::with(
            FactorInstancesCacheConflictResolutionStrategy::Panic, // Shoud not be triggered!
            file_system.clone(),
        );
        let sut2 = SUT::with(
            FactorInstancesCacheConflictResolutionStrategy::UseLatestInMemory {
                on_conflict_save_to_file: true,
            },
            file_system.clone(),
        );

        let fsid = FactorSourceIDFromHash::sample_at(0);

        let path = DerivationPreset::AccountVeci
            .index_agnostic_path_on_network(NetworkID::Mainnet);

        let two = Hardened::Unsecurified(UnsecurifiedHardened::TWO);
        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            two,
        );

        sut2.insert_single(&fi0).await.unwrap();

        // This is the FIRST time we access the cache in sut1, so we expect `load` to be called
        // which should pick up `two`
        let res = sut1.max_index_for(fsid, path).await.unwrap();
        assert_eq!(res, Some(HDPathComponent::from(two)));
    }
}

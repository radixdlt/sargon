use crate::prelude::*;
use std::borrow::Borrow;

/// A client which manages the cache of factor instances, by saving and loading from FileSystem
/// using a `FileSystemClient`.
///
/// This cache does not keep any state in memory, in order to avoid the possibility of
/// inconsistencies between the cache and the actual state of the filesystem.
#[derive(Debug, Clone)]
pub struct FactorInstancesCacheClient {
    file_system_client: Arc<FileSystemClient>,
}

impl FactorInstancesCacheClient {
    const CACHE_FILE: &'static str =
        "radix_babylon_wallet_pre_derived_public_keys_cache.json";

    pub fn new(file_system_client: Arc<FileSystemClient>) -> Self {
        Self { file_system_client }
    }

    async fn update_and_persist_cache<R>(
        &self,
        update: impl FnOnce(&mut FactorInstancesCache) -> Result<R>,
    ) -> Result<R> {
        let snapshot = self.load_from_file_or_default().await?;
        let mut cache = FactorInstancesCache::from(snapshot);
        let out = update(&mut cache)?;
        self.save_to_file(cache.serializable_snapshot()).await?;
        Ok(out)
    }

    async fn access_cache_init_if_needed<R>(
        &self,
        access: impl FnOnce(&FactorInstancesCache) -> Result<R>,
    ) -> Result<R> {
        let snapshot = self.load_from_file_or_default().await?;
        let cache = FactorInstancesCache::from(snapshot);
        access(&cache)
    }

    async fn load_from_file_or_default(
        &self,
    ) -> Result<FactorInstancesCacheSnapshot> {
        self.load_from_file()
            .await
            .map(|maybe_snapshot| maybe_snapshot.unwrap_or_default())
    }

    async fn load_from_file(
        &self,
    ) -> Result<Option<FactorInstancesCacheSnapshot>> {
        let path = self.path().await?;

        let maybe_json = self.file_system_client.load_from_file(path).await?;

        let Some(json) = maybe_json else {
            return Ok(None);
        };

        let deserialized =
            json.deserialize::<FactorInstancesCacheSnapshot>()?;

        Ok(Some(deserialized))
    }

    async fn path(&self) -> Result<String> {
        let path_str = self
            .file_system_client
            .create_if_needed(Self::CACHE_FILE)
            .await?;
        Ok(path_str)
    }

    async fn save_to_file(
        &self,
        cache_snapshot: FactorInstancesCacheSnapshot,
    ) -> Result<()> {
        let path = self.path().await?;
        let json = cache_snapshot.serialize_to_bytes()?;

        self.file_system_client
            .save_to_file(path, &json, true)
            .await
    }
}

impl FactorInstancesCacheClient {
    /// For tests
    pub async fn clear(&self) -> Result<()> {
        self.set_cache(FactorInstancesCacheSnapshot::default())
            .await
    }

    /// For tests
    pub async fn set_cache(
        &self,
        cache_snapshot: FactorInstancesCacheSnapshot,
    ) -> Result<()> {
        self.save_to_file(cache_snapshot).await
    }
}

impl FactorInstancesCacheClient {
    pub async fn delete(
        &self,
        instances_to_delete: impl Borrow<
            InstancesPerDerivationPresetPerFactorSource,
        >,
    ) -> Result<()> {
        self.update_and_persist_cache(|cache| {
            cache.delete(instances_to_delete.borrow());
            Ok(())
        })
        .await
    }

    pub async fn insert_for_factor(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        instances: impl Borrow<FactorInstances>,
    ) -> Result<bool> {
        self.update_and_persist_cache(|cache| {
            cache.insert_for_factor(&factor_source_id, instances.borrow())
        })
        .await
    }

    pub async fn snapshot(&self) -> Result<FactorInstancesCache> {
        self.access_cache_init_if_needed(|cache| Ok(cache.clone_snapshot()))
            .await
    }

    /// Inserts all instance in `per_factor`.
    pub async fn insert(
        &self,
        per_derivation_preset_per_factor: impl Borrow<
            InstancesPerDerivationPresetPerFactorSource,
        >,
    ) -> Result<()> {
        self.update_and_persist_cache(|cache| {
            cache.insert(per_derivation_preset_per_factor.borrow())
        })
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

    pub async fn get(
        &self,
        factor_source_ids: impl Borrow<IndexSet<FactorSourceIDFromHash>>,
        quantified_derivation_presets: impl Borrow<
            IdentifiedVecOf<QuantifiedDerivationPreset>,
        >,
        network_id: NetworkID,
    ) -> Result<CachedInstancesWithQuantitiesOutcome> {
        self.access_cache_init_if_needed(|cache| {
            cache.get(
                factor_source_ids.borrow(),
                quantified_derivation_presets.borrow(),
                network_id,
            )
        })
        .await
    }

    /// Reads out the instance of `factor_source_id` without mutating the cache.
    pub async fn peek_all_instances_of_factor_source(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Result<Option<IndexMap<IndexAgnosticPath, FactorInstances>>> {
        self.access_cache_init_if_needed(|cache| {
            Ok(cache.peek_all_instances_of_factor_source(factor_source_id))
        })
        .await
    }

    pub async fn total_number_of_factor_instances(&self) -> Result<usize> {
        self.access_cache_init_if_needed(|cache| {
            Ok(cache.total_number_of_factor_instances())
        })
        .await
    }
}

impl FactorInstancesCacheClient {
    /// Returns enough instances to satisfy the requested quantity for each factor source,
    /// **OR LESS**, never more, and if less, it means we MUST derive more, and if we
    /// must derive more, this function returns the quantities to derive for each factor source,
    /// for each derivation preset, not only the originally requested one.
    pub async fn get_poly_factor_with_quantities(
        &self,
        factor_source_ids: impl Borrow<IndexSet<FactorSourceIDFromHash>>,
        originally_requested_quantified_derivation_preset: impl Borrow<
            QuantifiedDerivationPreset,
        >,
        network_id: NetworkID,
    ) -> Result<CachedInstancesWithQuantitiesOutcome> {
        self.access_cache_init_if_needed(|cache| {
            cache.get_poly_factor_with_quantities(
                factor_source_ids.borrow(),
                originally_requested_quantified_derivation_preset.borrow(),
                network_id,
            )
        })
        .await
    }

    pub async fn insert_single(
        &self,
        instance: impl Borrow<HierarchicalDeterministicFactorInstance>,
    ) -> Result<bool> {
        let instance = instance.borrow();
        self.insert_for_factor(
            instance.factor_source_id,
            &FactorInstances::from_iter([instance.clone()]),
        )
        .await
    }

    pub fn in_memory() -> Self {
        Self {
            file_system_client: Arc::new(FileSystemClient::in_memory()),
        }
    }

    /// Queries the cache to see if the cache is full for factor_source_id for
    /// each DerivationPreset
    pub async fn is_full(
        &self,
        network_id: NetworkID,
        factor_source_id: FactorSourceIDFromHash,
    ) -> bool {
        self.access_cache_init_if_needed(|cache| {
            Ok(cache.is_full(network_id, factor_source_id))
        })
        .await
        .unwrap()
    }

    pub async fn assert_is_full(
        &self,
        network_id: NetworkID,
        factor_source_id: FactorSourceIDFromHash,
    ) {
        let is_full = self.is_full(network_id, factor_source_id).await;
        assert!(is_full);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorInstancesCacheClient;

    #[actix_rt::test]
    async fn test_load_modify_save_reload() {
        let file_system = Arc::new(FileSystemClient::in_memory());

        let sut1 = SUT::new(file_system.clone());

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

        sut1.insert_for_factor(fsid, &instances).await.unwrap();

        let network = NetworkID::Mainnet;
        let derivation_preset = DerivationPreset::AccountVeci;
        let path = derivation_preset.index_agnostic_path_on_network(network);
        let max = sut1.max_index_for(fsid, path).await.unwrap();
        assert_eq!(max, Some(one));

        let sut2 = SUT::new(file_system.clone());

        let max_sut2 = sut2.max_index_for(fsid, path).await.unwrap();
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

        let max_higher_sut1 = sut1.max_index_for(fsid, path).await.unwrap();
        let five = HDPathComponent::from(five);
        assert_eq!(max_higher_sut1, Some(five));

        sut2.delete(IndexMap::kv(
            DerivationPreset::AccountVeci,
            IndexMap::from_iter([(
                fsid,
                FactorInstances::from_iter([fi5.clone()]),
            )]),
        ))
        .await
        .unwrap();

        let max_higher_sut1 = sut1.max_index_for(fsid, path).await.unwrap();
        assert_eq!(max_higher_sut1, Some(one));

        // test get_poly_factor_with_quantities
        let poly = sut1
            .get_poly_factor_with_quantities(
                IndexSet::just(fsid),
                QuantifiedDerivationPreset::new(derivation_preset, 2),
                network,
            )
            .await
            .unwrap();

        let satisfied = IndexMap::kv(fsid, instances);

        pretty_assertions::assert_eq!(
            poly.into_satisfied()
                .unwrap()
                .cached
                .get(&DerivationPreset::AccountVeci)
                .cloned()
                .unwrap(),
            satisfied
        );

        let snap_1 = sut1.snapshot().await.unwrap();
        let snap_2 = sut2.snapshot().await.unwrap();
        assert_eq!(
            snap_1.serializable_snapshot(),
            snap_2.serializable_snapshot(),
        );
    }

    #[actix_rt::test]
    async fn test_insert_all() {
        let file_system = Arc::new(FileSystemClient::in_memory());
        let sut = SUT::new(file_system);

        let fs = FactorSourceIDFromHash::sample_at(0);
        sut.insert(&IndexMap::kv(
            DerivationPreset::AccountMfa,
            IndexMap::kv(fs, FactorInstances::sample()),
        ))
        .await
        .unwrap();

        let max = sut
            .max_index_for(
                fs,
                DerivationPreset::AccountMfa
                    .index_agnostic_path_on_network(NetworkID::Mainnet),
            )
            .await
            .unwrap();
        assert_eq!(
            max.unwrap(),
            HDPathComponent::Securified(SecurifiedU30::ONE)
        );
    }
}

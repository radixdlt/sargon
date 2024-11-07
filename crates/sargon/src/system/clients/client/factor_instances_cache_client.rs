use std::borrow::Borrow;

use crate::prelude::*;

#[derive(Debug)]
pub struct FactorInstancesCacheClient {
    file_system_client: Arc<FileSystemClient>,
    cache: RwLock<Option<FactorInstancesCache>>,
}

impl FactorInstancesCacheClient {
    const CACHE_FILE: &'static str = "factor_instances_cache.json";
    pub fn new(file_system_client: Arc<FileSystemClient>) -> Self {
        Self {
            file_system_client,
            cache: RwLock::new(None),
        }
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
        let mut cache = guard.as_mut().ok_or(CommonError::Unknown)?;
        update(&mut cache)
    }

    async fn init_if_needed(&self) -> Result<()> {
        {
            if self.cache.read().unwrap().is_some() {
                return Ok(());
            }
        }
        self.load().await
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

    async fn load(&self) -> Result<()> {
        {
            if self.cache.read().unwrap().is_some() {
                panic!("Cache already loaded");
            }
        }
        let json = self
            .file_system_client
            .load_from_file(Self::CACHE_FILE)
            .await?;
        let Some(json) = json else {
            return Err(CommonError::Unknown);
        };
        let cache_snapshot: FactorInstancesCacheSnapshot =
            serde_json::from_slice(json.as_slice())
                .map_err(|e| CommonError::Unknown)?;
        let cache = FactorInstancesCache::from(cache_snapshot);

        let mut guard = self.cache.write().unwrap();
        guard.replace(cache);

        Ok(())
    }

    async fn save(&self) -> Result<()> {
        let guard = self.cache.read().unwrap();
        let cache = guard.as_ref().ok_or(CommonError::Unknown)?;
        let serializable = cache.serializable_snapshot();
        let json = serde_json::to_vec(&serializable)
            .map_err(|e| CommonError::Unknown)?;

        self.file_system_client
            .save_to_file(Self::CACHE_FILE, &json)
            .await
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

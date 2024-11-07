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

    fn read<R>(&self, access: impl FnOnce(&FactorInstancesCache) -> Result<R>) -> Result<R> {
        let guard = self.cache.read().unwrap();
        let cache = guard.as_ref().ok_or(CommonError::Unknown)?;
        access(cache)
    }

    async fn load(&self) -> Result<()> {
        {
            if self.cache.read().unwrap().is_some() {
                return Ok(());
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
        todo!()
    }

    /// Inserts all instance in `per_factor`.
    pub async fn insert_all(
        &self,
        per_factor: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
    ) -> Result<()> {
        todo!()
    }
}

use crate::prelude::*;
use std::borrow::Borrow;

pub struct AccessControllerDetailsCacheClient {
    pub(crate) file_system_client: Arc<FileSystemClient>,
}

impl AccessControllerDetailsCacheClient {
    const CACHE_FILE: &'static str =
        "radix_babylon_wallet_access_controller_details_cache.json";

    pub fn new(file_system_client: Arc<FileSystemClient>) -> Self {
        Self { file_system_client }
    }

    async fn update_and_persist_cache<R>(
        &self,
        update: impl FnOnce(&mut AccessControllerDetailsCache) -> Result<R>,
    ) -> Result<R> {
        let snapshot = self.load_from_file_or_default().await?;
        let mut cache = AccessControllerDetailsCache::from(snapshot);
        let out = update(&mut cache)?;
        self.save_to_file(cache.serializable_snapshot()).await?;
        Ok(out)
    }

    async fn access_cache<R>(
        &self,
        access: impl FnOnce(&AccessControllerDetailsCache) -> Result<R>,
    ) -> Result<R> {
        let snapshot = self.load_from_file_or_default().await?;
        let cache = AccessControllerDetailsCache::from(snapshot);
        access(&cache)
    }

    async fn load_from_file_or_default(
        &self,
    ) -> Result<AccessControllerDetailsCacheSnapshot> {
        self.load_from_file()
            .await
            .map(|maybe_snapshot| maybe_snapshot.unwrap_or_default())
    }

    async fn load_from_file(
        &self,
    ) -> Result<Option<AccessControllerDetailsCacheSnapshot>> {
        let path = self.path().await?;

        let maybe_json = self.file_system_client.load_from_file(path).await?;

        let Some(json) = maybe_json else {
            return Ok(None);
        };

        let deserialized =
            json.deserialize::<AccessControllerDetailsCacheSnapshot>()?;

        Ok(Some(deserialized))
    }

    async fn path(&self) -> Result<String> {
        self.file_system_client
            .create_if_needed(Self::CACHE_FILE)
            .await
    }

    async fn save_to_file(
        &self,
        cache_snapshot: AccessControllerDetailsCacheSnapshot,
    ) -> Result<()> {
        let path = self.path().await?;
        let json = cache_snapshot.serialize_to_bytes()?;

        self.file_system_client
            .save_to_file(path, &json, true)
            .await
    }
}

impl AccessControllerDetailsCacheClient {
    pub async fn insert(
        &self,
        details: impl Borrow<AccessControllerStateDetails>,
    ) -> Result<()> {
        self.update_and_persist_cache(|cache| cache.insert(details.borrow()))
            .await
    }

    pub async fn insert_many(
        &self,
        details: Vec<impl Borrow<AccessControllerStateDetails>>,
    ) -> Result<()> {
        self.update_and_persist_cache(|cache| {
            for item in details {
                cache.insert(item.borrow())?;
            }
            Ok(())
        })
        .await
    }

    pub async fn snapshot(&self) -> Result<AccessControllerDetailsCache> {
        self.access_cache(|cache| Ok(cache.clone())).await
    }
}

impl AccessControllerDetailsCacheClient {
    /// For tests
    pub async fn clear(&self) -> Result<()> {
        self.set_cache(AccessControllerDetailsCacheSnapshot::default())
            .await
    }

    /// For tests
    pub async fn set_cache(
        &self,
        cache_snapshot: AccessControllerDetailsCacheSnapshot,
    ) -> Result<()> {
        self.save_to_file(cache_snapshot).await
    }
}

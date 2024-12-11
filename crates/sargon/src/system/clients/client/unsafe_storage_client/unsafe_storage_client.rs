use crate::prelude::*;

#[derive(Debug)]
pub struct UnsafeStorageClient {
    #[allow(dead_code)]
    driver: Arc<dyn UnsafeStorageDriver>,
}

impl UnsafeStorageClient {
    pub(crate) fn new(driver: Arc<dyn UnsafeStorageDriver>) -> Self {
        Self { driver }
    }
}

impl UnsafeStorageClient {
    //======
    // Load T
    //======
    /// Loads bytes from UnsafeStorageDriver and deserializes them into `T`.
    ///
    /// Returns `Ok(None)` if no bytes were found, returns Err if failed
    /// to load bytes or failed to deserialize the JSON into a `T`.
    pub async fn load<T>(&self, key: UnsafeStorageKey) -> Result<Option<T>>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.driver.load_data(key).await.and_then(|o| match o {
            None => Ok(None),
            Some(j) => serde_json::from_slice(j.as_slice())
                .map_failed_to_deserialize_bytes::<T>(j.as_slice()),
        })
    }

    /// Loads bytes from UnsafeStorageDriver and deserializes them into `T`.
    ///
    /// Returns Err if failed to load bytes or failed to deserialize the JSON into a `T`,
    /// unlike `load` this method returns `default` if `None` bytes were found.
    pub async fn load_unwrap_or<T>(
        &self,
        key: UnsafeStorageKey,
        default: T,
    ) -> T
    where
        T: for<'a> serde::Deserialize<'a> + Clone,
    {
        self.load(key)
            .await
            .map(|o| o.unwrap_or(default.clone()))
            .unwrap_or(default)
    }

    //======
    // Backup up mnemonics
    //======
    pub async fn check_if_mnemonic_is_backed_up(
        &self,
        factor_source: DeviceFactorSource,
    ) -> Result<bool> {
        let result: Vec<FactorSourceIDFromHash> = self
            .load_unwrap_or(
                UnsafeStorageKey::FactorSourceUserHasWrittenDown,
                Vec::new(),
            )
            .await;
        Ok(result.contains(&factor_source.id))
    }
}

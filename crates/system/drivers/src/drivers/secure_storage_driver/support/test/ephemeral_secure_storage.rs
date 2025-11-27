use crate::prelude::*;

use std::sync::RwLock;

/// Used for testing - a type which saves into memory.
#[derive(Debug)]
pub struct EphemeralSecureStorage {
    pub storage: RwLock<HashMap<SecureStorageKey, BagOfBytes>>,
}

impl EphemeralSecureStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(EphemeralSecureStorage {
            storage: RwLock::new(HashMap::new()),
        })
    }
}

#[async_trait::async_trait]
impl SecureStorageDriver for EphemeralSecureStorage {
    async fn load_data(
        &self,
        key: SecureStorageKey,
    ) -> Result<Option<BagOfBytes>> {
        self.storage
            .try_read()
            .map_err(|_| CommonError::SecureStorageReadError {
                underlying: "Failed to read storage".to_string(),
            })
            .map(|s| s.get(&key).cloned())
    }

    async fn save_data(
        &self,
        key: SecureStorageKey,
        value: BagOfBytes,
    ) -> Result<()> {
        let mut storage = self.storage.try_write().map_err(|_| {
            CommonError::SecureStorageWriteError {
                underlying: "Failed to write storage".to_string(),
            }
        })?;

        storage.insert(key, value);
        Ok(())
    }

    async fn delete_data_for_key(&self, key: SecureStorageKey) -> Result<()> {
        let mut storage = self.storage.try_write().map_err(|_| {
            CommonError::SecureStorageWriteError {
                underlying: "Failed to write storage".to_string(),
            }
        })?;

        storage.remove_entry(&key);
        Ok(())
    }

    async fn contains_data_for_key(
        &self,
        key: SecureStorageKey,
    ) -> Result<bool> {
        self.storage
            .try_read()
            .map_err(|_| CommonError::SecureStorageReadError {
                underlying: "Failed to read storage".to_string(),
            })
            .map(|s| s.contains_key(&key))
    }
}

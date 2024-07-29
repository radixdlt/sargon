#![cfg(test)]
use crate::prelude::*;

use std::sync::RwLock;

/// Used for testing - a type which saves into memory.
#[derive(Debug)]
pub struct EphemeralUnsafeStorage {
    pub storage: RwLock<HashMap<UnsafeStorageKey, BagOfBytes>>,
}

impl EphemeralUnsafeStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(EphemeralUnsafeStorage {
            storage: RwLock::new(HashMap::new()),
        })
    }
}

impl UnsafeStorageDriver for EphemeralUnsafeStorage {
    fn load_data(&self, key: UnsafeStorageKey) -> Result<Option<BagOfBytes>> {
        self.storage
            .try_read()
            .map_err(|_| CommonError::UnsafeStorageReadError)
            .map(|s| s.get(&key).cloned())
    }

    fn save_data(
        &self,
        key: UnsafeStorageKey,
        value: BagOfBytes,
    ) -> Result<()> {
        let mut storage = self
            .storage
            .try_write()
            .map_err(|_| CommonError::UnsafeStorageWriteError)?;

        storage.insert(key, value);
        Ok(())
    }

    fn delete_data_for_key(&self, key: UnsafeStorageKey) -> Result<()> {
        let mut storage = self
            .storage
            .try_write()
            .map_err(|_| CommonError::UnsafeStorageWriteError)?;

        storage.remove_entry(&key);
        Ok(())
    }
}

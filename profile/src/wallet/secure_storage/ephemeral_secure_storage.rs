#![cfg(test)]
use crate::prelude::*;

use std::sync::RwLock;

/// Used for testing - a type which saves into memory.
#[derive(Debug)]
pub struct EphemeralSecureStorage {
    pub storage: RwLock<HashMap<SecureStorageKey, Vec<u8>>>,
}
impl EphemeralSecureStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(EphemeralSecureStorage {
            storage: RwLock::new(HashMap::new()),
        })
    }
}

impl SecureStorage for EphemeralSecureStorage {
    fn load_data(&self, key: SecureStorageKey) -> Result<Option<Vec<u8>>> {
        self.storage
            .try_read()
            .map_err(|_| CommonError::SecureStorageReadError)
            .and_then(|s| Ok(s.get(&key).cloned()))
    }

    fn save_data(&self, key: SecureStorageKey, value: Vec<u8>) -> Result<()> {
        let mut storage = self
            .storage
            .try_write()
            .map_err(|_| CommonError::SecureStorageWriteError)?;

        storage.insert(key, value);
        Ok(())
    }

    fn delete_data_for_key(&self, key: SecureStorageKey) -> Result<()> {
        let mut storage = self
            .storage
            .try_write()
            .map_err(|_| CommonError::SecureStorageWriteError)?;

        storage.remove_entry(&key);
        Ok(())
    }
}

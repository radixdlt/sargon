#![cfg(test)]
use crate::prelude::*;

/// Used for testing - a type which does NOT save anything.
#[derive(Debug)]
pub struct EphemeralSecureStorage {}
impl EphemeralSecureStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(EphemeralSecureStorage {})
    }
}
impl SecureStorage for EphemeralSecureStorage {
    fn load_data(&self, _key: SecureStorageKey) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn save_data(&self, _key: SecureStorageKey, _value: Vec<u8>) -> Result<()> {
        Ok(())
    }
}

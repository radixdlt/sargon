use std::sync::Arc;

use crate::{SecureStorageKey, SecureStorage, CommonError};

#[derive(Debug)]
pub struct MockSecureStorage {}
impl MockSecureStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(MockSecureStorage {})
    }
}
impl SecureStorage for MockSecureStorage {
    fn load_data(&self, key: SecureStorageKey) -> Result<Option<Vec<u8>>, CommonError> {
        panic!("You have not installed any secure storage yet.")
    }

    fn save_data(&self, key: SecureStorageKey, value: Vec<u8>) -> Result<(), CommonError> {
        panic!("You have not installed any secure storage yet.")
    }
}

#![cfg(test)]

use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct AlwaysFailStorage {}

impl SecureStorage for AlwaysFailStorage {
    fn load_data(&self, _key: SecureStorageKey) -> Result<Option<Vec<u8>>> {
        panic!("AlwaysFailStorage does not implement `load_data");
    }

    fn save_data(&self, _key: SecureStorageKey, _data: Vec<u8>) -> Result<()> {
        Err(CommonError::Unknown)
    }

    fn delete_data_for_key(&self, _key: SecureStorageKey) -> Result<()> {
        panic!("AlwaysFailStorage does not implement `delete_data_for_key");
    }
}

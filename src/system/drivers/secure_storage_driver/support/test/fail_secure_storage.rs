#![cfg(test)]

use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct AlwaysFailStorage {}

#[async_trait::async_trait]
impl SecureStorageDriver for AlwaysFailStorage {
    async fn load_data(
        &self,
        _key: SecureStorageKey,
    ) -> Result<Option<Vec<u8>>> {
        panic!("AlwaysFailStorage does not implement `load_data");
    }

    async fn save_data(
        &self,
        _key: SecureStorageKey,
        _data: Vec<u8>,
    ) -> Result<()> {
        Err(CommonError::Unknown)
    }

    async fn delete_data_for_key(&self, _key: SecureStorageKey) -> Result<()> {
        panic!("AlwaysFailStorage does not implement `delete_data_for_key");
    }
}

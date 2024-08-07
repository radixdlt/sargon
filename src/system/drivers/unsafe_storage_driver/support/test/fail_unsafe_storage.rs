#![cfg(test)]

use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct AlwaysFailUnsafeStorage {}

#[async_trait::async_trait]
impl UnsafeStorageDriver for AlwaysFailUnsafeStorage {
    async fn load_data(
        &self,
        _key: UnsafeStorageKey,
    ) -> Result<Option<BagOfBytes>> {
        panic!("AlwaysFailStorage does not implement `load_data");
    }

    async fn save_data(
        &self,
        _key: UnsafeStorageKey,
        _data: BagOfBytes,
    ) -> Result<()> {
        Err(CommonError::Unknown)
    }

    async fn delete_data_for_key(&self, _key: UnsafeStorageKey) -> Result<()> {
        panic!("AlwaysFailStorage does not implement `delete_data_for_key");
    }
}

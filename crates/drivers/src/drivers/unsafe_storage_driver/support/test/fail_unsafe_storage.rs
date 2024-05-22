// #![cfg(test)] // FIXME: multi crate cf test

use crate::prelude::*;

#[derive(Debug)]
pub struct AlwaysFailUnsafeStorage {}

impl UnsafeStorageDriver for AlwaysFailUnsafeStorage {
    fn load_data(&self, _key: UnsafeStorageKey) -> Result<Option<BagOfBytes>> {
        panic!("AlwaysFailStorage does not implement `load_data");
    }

    fn save_data(
        &self,
        _key: UnsafeStorageKey,
        _data: BagOfBytes,
    ) -> Result<()> {
        Err(CommonError::Unknown)
    }

    fn delete_data_for_key(&self, _key: UnsafeStorageKey) -> Result<()> {
        panic!("AlwaysFailStorage does not implement `delete_data_for_key");
    }
}

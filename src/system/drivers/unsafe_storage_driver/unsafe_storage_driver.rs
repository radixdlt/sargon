use crate::prelude::*;

#[uniffi::export(with_foreign)]
pub trait UnsafeStorageDriver: Send + Sync + std::fmt::Debug {
    fn load_data(&self, key: UnsafeStorageKey) -> Result<Option<BagOfBytes>>;

    fn save_data(&self, key: UnsafeStorageKey, data: BagOfBytes) -> Result<()>;

    fn delete_data_for_key(&self, key: UnsafeStorageKey) -> Result<()>;
}

use crate::prelude::*;

#[derive(Debug)]
pub struct AlwaysFailSecureStorage {}

#[async_trait::async_trait]
impl SecureStorageDriver for AlwaysFailSecureStorage {
    async fn load_data(
        &self,
        _key: SecureStorageKey,
    ) -> Result<Option<BagOfBytes>> {
        panic!("AlwaysFailStorage does not implement `load_data");
    }

    async fn save_data(
        &self,
        _key: SecureStorageKey,
        _data: BagOfBytes,
    ) -> Result<()> {
        Err(CommonError::Unknown {
            error_message: "Failed saving data".to_string(),
        })
    }

    async fn delete_data_for_key(&self, _key: SecureStorageKey) -> Result<()> {
        panic!("AlwaysFailStorage does not implement `delete_data_for_key");
    }

    async fn contains_data_for_key(
        &self,
        _key: SecureStorageKey,
    ) -> Result<bool> {
        panic!("AlwaysFailStorage does not implement `contains_data_for_key");
    }
}

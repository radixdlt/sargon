#![cfg(test)]

use crate::prelude::*;

#[derive(Debug)]
pub struct MockSecureStorage {
    mock_contains_data_for_key: IndexMap<SecureStorageKey, bool>,
}

impl MockSecureStorage {
    pub fn new(
        mock_contains_data_for_key: IndexMap<SecureStorageKey, bool>,
    ) -> Self {
        Self {
            mock_contains_data_for_key,
        }
    }
}

#[async_trait::async_trait]
impl SecureStorageDriver for MockSecureStorage {
    async fn load_data(
        &self,
        _key: SecureStorageKey,
    ) -> Result<Option<BagOfBytes>> {
        Ok(None)
    }

    async fn save_data(
        &self,
        _key: SecureStorageKey,
        _data: BagOfBytes,
    ) -> Result<()> {
        Ok(())
    }

    async fn delete_data_for_key(&self, _key: SecureStorageKey) -> Result<()> {
        Ok(())
    }

    async fn contains_data_for_key(
        &self,
        key: SecureStorageKey,
    ) -> Result<bool> {
        let value = self
            .mock_contains_data_for_key
            .get(&key)
            .copied()
            .ok_or(CommonError::Unknown)?;
        Ok(value)
    }
}

use crate::{SecureStorageKey, CommonError};

#[uniffi::export]
pub trait SecureStorage: Send + Sync + std::fmt::Debug {
    fn load_data(&self, key: SecureStorageKey) -> Result<Option<Vec<u8>>, CommonError>;
    fn save_data(&self, key: SecureStorageKey, data: Vec<u8>) -> Result<(), CommonError>;
}

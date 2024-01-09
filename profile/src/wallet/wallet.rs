use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use uuid::Uuid;

use crate::{CommonError, FactorSourceIDFromHash, Profile};

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum SecureStorageKey {
    DeviceFactorSourceMnemonic {
        factor_source_id: FactorSourceIDFromHash,
    },
    ProfileSnapshot {
        profile_id: Uuid,
    },
}
impl SecureStorageKey {
    pub fn identifier(&self) -> String {
        format!(
            "secure_storage_key_{}",
            match self {
                SecureStorageKey::DeviceFactorSourceMnemonic { factor_source_id } =>
                    format!("device_factor_source_{}", factor_source_id.to_string()),
                SecureStorageKey::ProfileSnapshot { profile_id } =>
                    format!("profile_snapshot_{}", profile_id),
            }
        )
    }
}

#[uniffi::export]
pub fn secure_storage_key_identifier(key: &SecureStorageKey) -> String {
    key.identifier()
}

#[uniffi::export]
pub trait SecureStorage: Send + Sync + std::fmt::Debug {
    fn get(&self, key: SecureStorageKey) -> Result<Option<String>, CommonError>;
    fn put(&self, key: SecureStorageKey, value: String) -> Result<(), CommonError>;
}

#[derive(Debug)]
pub struct NotYetSetSecureStorage {}
impl NotYetSetSecureStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(NotYetSetSecureStorage {})
    }
}
impl SecureStorage for NotYetSetSecureStorage {
    fn get(&self, key: SecureStorageKey) -> Result<Option<String>, CommonError> {
        panic!("You have not installed any secure storage yet.")
    }

    fn put(&self, key: SecureStorageKey, value: String) -> Result<(), CommonError> {
        panic!("You have not installed any secure storage yet.")
    }
}

#[derive(Debug, uniffi::Object)]
pub struct Wallet {
    profile: RwLock<Profile>,
    pub(crate) read_secure_storage: Arc<dyn SecureStorage>,
}

//========
// CONSTRUCTOR
//========
#[uniffi::export]
impl Wallet {
    #[uniffi::constructor]
    pub fn new(profile: Profile, secure_storage: Arc<dyn SecureStorage>) -> Self {
        Self {
            profile: RwLock::new(profile),
            read_secure_storage: secure_storage,
        }
    }
}

//========
// GET
//========
#[uniffi::export]
impl Wallet {
    /// Takes a snapshot of the profile and serialize it as a String of JSON.
    pub fn json_snapshot(&self) -> String {
        serde_json::to_string(&self.profile())
            .expect("Should always be able to JSON serialize a Profile.")
    }

    /// Clone the profile and return it.
    pub fn profile(&self) -> Profile {
        self.read(|p| p.clone())
    }
}

impl Wallet {
    pub(crate) fn read<T: Clone, F>(&self, access: F) -> T
    where
        F: Fn(RwLockReadGuard<'_, Profile>) -> T,
    {
        self.profile
            .try_read()
            .map(access)
            .expect("Implementing Wallet clients should not read and write Profile from Wallet from multiple threads.")
    }

    pub(crate) fn write<F, R>(&self, mutate: F) -> R
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> R,
    {
        self.profile
            .try_write()
            .map(mutate)
            .expect("Implementing Wallet clients should not read and write Profile from Wallet from multiple threads.")
    }
}

#[cfg(test)]
mod tests {
    use crate::{HasPlaceholder, Profile};

    use super::{NotYetSetSecureStorage, Wallet};

    #[test]
    fn read_header() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone(), NotYetSetSecureStorage::new());
        wallet.read(|p| assert_eq!(p.header, profile.header))
    }

    #[test]
    fn take_snapshot() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone(), NotYetSetSecureStorage::new());
        assert_eq!(wallet.profile(), profile)
    }
}

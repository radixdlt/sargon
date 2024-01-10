use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use identified_vec::{Identifiable, IsIdentifiedVec};
use uuid::Uuid;

use crate::{CommonError, FactorSourceIDFromHash, Header, IdentifiedVecVia, Profile, ProfileID};

pub type HeadersList = IdentifiedVecVia<Header>;
impl Identifiable for Header {
    type ID = ProfileID;

    fn id(&self) -> Self::ID {
        self.id.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum SecureStorageKey {
    SnapshotHeadersList,
    DeviceFactorSourceMnemonic {
        factor_source_id: FactorSourceIDFromHash,
    },
    ProfileSnapshot {
        profile_id: ProfileID,
    },
}
impl SecureStorageKey {
    pub fn identifier(&self) -> String {
        format!(
            "secure_storage_key_{}",
            match self {
                SecureStorageKey::SnapshotHeadersList => "headers".to_string(),
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

pub type Data = Vec<u8>;

#[uniffi::export]
pub trait SecureStorage: Send + Sync + std::fmt::Debug {
    fn load_data(&self, key: SecureStorageKey) -> Result<Option<Data>, CommonError>;
    fn save_data(&self, key: SecureStorageKey, data: Data) -> Result<(), CommonError>;
}

#[derive(Debug)]
pub struct NotYetSetSecureStorage {}
impl NotYetSetSecureStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(NotYetSetSecureStorage {})
    }
}
impl SecureStorage for NotYetSetSecureStorage {
    fn load_data(&self, key: SecureStorageKey) -> Result<Option<Data>, CommonError> {
        panic!("You have not installed any secure storage yet.")
    }

    fn save_data(&self, key: SecureStorageKey, value: Data) -> Result<(), CommonError> {
        panic!("You have not installed any secure storage yet.")
    }
}

#[derive(Debug)]
pub struct WalletClientStorage {
    interface: Arc<dyn SecureStorage>,
}
impl WalletClientStorage {
    pub(crate) fn new(interface: Arc<dyn SecureStorage>) -> Self {
        Self { interface }
    }
}
impl WalletClientStorage {
    pub fn load<'de, T>(&self, key: SecureStorageKey) -> Result<Option<T>, CommonError>
    where
        T: serde::Deserialize<'de>,
    {
        self.interface.load_data(key).and_then(|o| match o {
            None => Ok(None),
            Some(j) => {
                serde_json::from_slice(&j).map_err(|e| CommonError::FailedToDeserializeToJSON)
            }
        })
    }

    /// Like `load` but returns `Result<T>` instead of `Result<Option<T>>` and throws the provided error if
    /// the value was `None`.
    pub fn load_or<'de, T>(&self, key: SecureStorageKey, err: CommonError) -> Result<T, CommonError>
    where
        T: serde::Deserialize<'de>,
    {
        self.load(key).and_then(|o| o.ok_or(err))
    }

    /// Like `load` but returns `T` instead of `Result<Option<T>>` and defaults to `default`, if `load` returned `Ok(None)` or `Err`.
    pub fn load_unwrap_or<'de, T>(&self, key: SecureStorageKey, default: T) -> T
    where
        T: serde::Deserialize<'de>,
    {
        self.load(key)
            .map(|o| o.unwrap_or(default))
            .unwrap_or(default)
    }

    pub fn load_headers_list_or_empty(&self) -> HeadersList {
        self.load_unwrap_or(SecureStorageKey::SnapshotHeadersList, HeadersList::new())
    }
}

/// Panics and logs with error the `reason` (with file/line context.)
pub fn log_panic(prefix: &str, provided_reason: impl AsRef<str>) {
    let msg = format!(
        "{}: '{}' ({}:{}:{})",
        prefix,
        provided_reason.as_ref(),
        file!(),
        line!(),
        column!()
    );
    log::error!("{}", msg);
    panic!("{}", msg);
}
pub fn incorrect_impl(reason: impl AsRef<str>) {
  log_panic("Incorrect implementation", reason)
}
pub fn fatal_error(reason: impl AsRef<str>) {
  log_panic("Fatal error", reason)
}

impl WalletClientStorage {
    pub(crate) fn assert_not_contains_profile_with_id(&self, profile_id: ProfileID) {
        if self.load_headers_list_or_empty().contains_id(&profile_id) {
            fatal_error(format!("Profile with id {profile_id}"))
        }
    }

    pub fn save<T>(&self, key: SecureStorageKey, value: &T) -> Result<(), CommonError>
    where
        T: serde::Serialize,
    {
        serde_json::to_vec(value)
            .map_err(|e| CommonError::FailedToSerializeToJSON)
            .and_then(|j| self.interface.save_data(key, j))
    }
}

#[derive(Debug, uniffi::Object)]
pub struct Wallet {
    profile: RwLock<Profile>,
    pub(crate) wallet_client_storage: WalletClientStorage,
}

impl Wallet {
    /// Initializes logging
    fn init_logging(&self) {
        env_logger::init();
    }
    /// Initializes the wallet, setting up e.g. logging
    fn init(&self) {
        self.init_logging()
    }
}

//========
// CONSTRUCTOR
//========
#[uniffi::export]
impl Wallet {
    /// Creates wallet with an entirely new Profile, this function panics if the profile already exists.
    #[uniffi::constructor]
    pub fn new(profile: Profile, secure_storage: Arc<dyn SecureStorage>) -> Self {
        // Init WalletClient's storage
        let wallet_client_storage = WalletClientStorage::new(secure_storage);
        // profile.id() MUST be new, clients should not call `Wallet::new` with an existing Profile, they MUST
        // use `Wallet::with_existing_profile` for existing profiles (they should check first..)
        wallet_client_storage.assert_not_contains_profile_with_id(profile.id());
        let wallet = Self {
            profile: RwLock::new(profile),
            wallet_client_storage,
        };
        wallet.save_profile_or_log(&profile);
        wallet.init();
        wallet
    }

    #[uniffi::constructor]
    pub fn with_existing_profile(
        secure_storage: Arc<dyn SecureStorage>,
    ) -> Result<Self, CommonError> {
        // wallet.init();
        // secure_storage.get(SecureStorageKey::)
        todo!()
    }
}

//========
// Wallet + SecureStorage
//========
impl Wallet {
    pub(crate) fn save_profile(&self, profile: &Profile) -> Result<(), CommonError> {
        self.wallet_client_storage.save(
            SecureStorageKey::ProfileSnapshot {
                profile_id: profile.header.id,
            },
            profile,
        )
    }
    pub(crate) fn save_profile_or_log(&self, profile: &Profile) {
        match self.save_profile(profile) {
            Ok(_) => log::info!("Successfully saved profile with ID: {}", profile.id()),
            Err(e) => log::error!(
                "Failed to save profile with ID: {}, error: {}",
                profile.id(),
                e
            ),
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

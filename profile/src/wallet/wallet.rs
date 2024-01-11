use crate::prelude::*;
use identified_vec::Identifiable;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type HeadersList = IdentifiedVecVia<Header>;
impl Identifiable for Header {
    type ID = ProfileID;

    fn id(&self) -> Self::ID {
        self.id.clone()
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
    pub fn with_existing_profile(secure_storage: Arc<dyn SecureStorage>) -> Result<Self> {
        // wallet.init();
        // secure_storage.get(SecureStorageKey::)
        todo!()
    }
}

//========
// Wallet + SecureStorage
//========
impl Wallet {
    pub(crate) fn save_profile(&self, profile: &Profile) -> Result<()> {
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
    use crate::{HasPlaceholder, MockSecureStorage, Profile, Wallet};

    #[test]
    fn read_header() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone(), MockSecureStorage::new());
        wallet.read(|p| assert_eq!(p.header, profile.header))
    }

    #[test]
    fn take_snapshot() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone(), MockSecureStorage::new());
        assert_eq!(wallet.profile(), profile)
    }
}

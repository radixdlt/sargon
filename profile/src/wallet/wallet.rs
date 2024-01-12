use crate::prelude::*;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

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

        // Init wallet
        let wallet = Self {
            profile: RwLock::new(profile.clone()),
            wallet_client_storage,
        };

        // Save new profile (also sets activeProfileID)
        wallet.save_new_profile_or_panic(&profile);

        // Init logging
        wallet.init_logging();
        wallet
    }

    #[uniffi::constructor]
    pub fn with_existing_profile(secure_storage: Arc<dyn SecureStorage>) -> Result<Self> {
        // Init WalletClient's storage
        let wallet_client_storage = WalletClientStorage::new(secure_storage);

        // Load active profile ID
        let active_profile_id: ProfileID = wallet_client_storage.load_or(
            SecureStorageKey::ActiveProfileID,
            CommonError::NoActiveProfileIDSet,
        )?;

        // Form storage key
        let profile_key = SecureStorageKey::ProfileSnapshot {
            profile_id: active_profile_id.clone(),
        };

        // Load Profile from storage with key
        let profile: Profile = wallet_client_storage.load_or(
            profile_key,
            CommonError::ProfileSnapshotNotFound(active_profile_id),
        )?;

        // Create wallet
        let wallet = Self {
            profile: RwLock::new(profile),
            wallet_client_storage,
        };

        // Set active profile ID
        wallet.save_active_profile_id(&profile_id())?;

        // Init logging
        wallet.init_logging();

        Ok(wallet)
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
    use crate::prelude::*;
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

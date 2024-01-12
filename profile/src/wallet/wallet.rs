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
        // env_logger::init();
        // Hmm ^^ not needed? already initialized?
        // env_logger::init should not be called after logger initialized: SetLoggerError(())
    }
}

//========
// CONSTRUCTOR
//========
#[uniffi::export]
impl Wallet {
    /// Creates a new Mnemonic from `entropy` (without BIP39 passphrase) and creates a new Profile,
    /// saving both the Mnemonic and Profile into secure storage and returns a new Wallet.
    #[uniffi::constructor]
    pub fn by_creating_new_profile_and_secrets_with_entropy(
        entropy: Vec<u8>,
        wallet_client_model: WalletClientModel,
        wallet_client_name: String,
        secure_storage: Arc<dyn SecureStorage>,
    ) -> Result<Self> {
        let entropy_32bytes = Hex32Bytes::from_vec(entropy)?;
        let private_hd_factor_source =
            PrivateHierarchicalDeterministicFactorSource::new_with_entropy(
                entropy_32bytes,
                wallet_client_model,
            );

        let profile = Profile::new(
            private_hd_factor_source.clone(),
            wallet_client_name.as_str(),
        );
        let wallet = Self::by_importing_profile(profile, secure_storage);
        wallet.wallet_client_storage.save(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: private_hd_factor_source.factor_source.id.clone(),
            },
            &private_hd_factor_source.mnemonic_with_passphrase,
        )?;
        Ok(wallet)
    }

    /// Creates wallet by *importing* a Profile.
    #[uniffi::constructor]
    pub fn by_importing_profile(profile: Profile, secure_storage: Arc<dyn SecureStorage>) -> Self {
        // Init WalletClient's storage
        let wallet_client_storage = WalletClientStorage::new(secure_storage);

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
    pub fn by_loading_profile(secure_storage: Arc<dyn SecureStorage>) -> Result<Self> {
        // Init WalletClient's storage
        let wallet_client_storage = WalletClientStorage::new(secure_storage);

        // Load active profile ID
        let active_profile_id: ProfileID = wallet_client_storage.load_or(
            SecureStorageKey::ActiveProfileID,
            CommonError::NoActiveProfileIDSet,
        )?;

        Self::new_load_profile_with_id(active_profile_id, wallet_client_storage)
    }

    #[uniffi::constructor]
    pub fn by_loading_profile_with_id(
        profile_id: ProfileID,
        secure_storage: Arc<dyn SecureStorage>,
    ) -> Result<Self> {
        Self::new_load_profile_with_id(profile_id, WalletClientStorage::new(secure_storage))
    }
}

impl Wallet {
    fn new_load_profile_with_id(
        profile_id: ProfileID,
        wallet_client_storage: WalletClientStorage,
    ) -> Result<Self> {
        // Form storage key
        let profile_key = SecureStorageKey::ProfileSnapshot {
            profile_id: profile_id.clone(),
        };

        // Load Profile from storage with key
        let profile: Profile = wallet_client_storage.load_or(
            profile_key,
            CommonError::ProfileSnapshotNotFound(profile_id.clone()),
        )?;

        // Create wallet
        let wallet = Self {
            profile: RwLock::new(profile),
            wallet_client_storage,
        };

        // Set active profile ID
        wallet.save_active_profile_id(&profile_id)?;

        // Init logging
        wallet.init_logging();

        Ok(wallet)
    }
}

#[cfg(test)]
impl Wallet {
    pub(crate) fn ephemeral(profile: Profile) -> Self {
        Self::by_importing_profile(profile, EphemeralSecureStorage::new())
    }
}
#[cfg(test)]
impl HasPlaceholder for Wallet {
    fn placeholder() -> Self {
        Self::ephemeral(Profile::placeholder())
    }

    fn placeholder_other() -> Self {
        Self::ephemeral(Profile::placeholder_other())
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
        let wallet = Wallet::ephemeral(profile.clone());
        wallet.read(|p| assert_eq!(p.header, profile.header))
    }

    #[test]
    fn take_snapshot() {
        let profile = Profile::placeholder();
        let wallet = Wallet::ephemeral(profile.clone());
        assert_eq!(wallet.profile(), profile)
    }
}

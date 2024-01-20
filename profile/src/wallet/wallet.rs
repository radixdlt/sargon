use crate::prelude::*;
use std::sync::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type HeadersList = IdentifiedVecVia<Header>;

#[derive(Debug, uniffi::Object)]
pub struct Wallet {
    // This is pub(crate) for testing purposes only, i.e. causing the RwLock to be poisoned.
    pub(crate) profile: RwLock<Profile>,
    pub(crate) wallet_client_storage: WalletClientStorage,
}

impl Wallet {
    /// Initializes logging
    fn init_logging() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            pretty_env_logger::formatted_builder()
                .filter_level(log::LevelFilter::Trace)
                .try_init()
                .expect("Should be able to setup a logger.");
        });
    }

    fn with_imported_profile(profile: Profile, secure_storage: Arc<dyn SecureStorage>) -> Self {
        // Init WalletClient's storage
        let wallet_client_storage = WalletClientStorage::new(secure_storage);

        // Init wallet
        let wallet = Self {
            profile: RwLock::new(profile.clone()),
            wallet_client_storage,
        };

        // Save new profile (also sets activeProfileID)
        wallet.save_new_profile_or_panic(&profile);

        wallet
    }

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

        Ok(wallet)
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
        Wallet::init_logging();

        log::info!("Instantiating Wallet by creating a new Profile from entropy (provided), for client: {}", wallet_client_model);

        let entropy_32bytes = Hex32Bytes::from_vec(entropy)?;
        let private_hd_factor_source =
            PrivateHierarchicalDeterministicFactorSource::new_with_entropy(
                entropy_32bytes,
                BIP39Passphrase::default(),
                wallet_client_model,
            );

        let profile = Profile::new(
            private_hd_factor_source.clone(),
            wallet_client_name.as_str(),
        );
        let wallet = Self::with_imported_profile(profile, secure_storage);
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
        Wallet::init_logging();

        log::info!(
            "Instantiating Wallet by importing a Profile with ID: {}",
            profile.id()
        );

        Self::with_imported_profile(profile, secure_storage)
    }

    #[uniffi::constructor]
    pub fn by_loading_profile(secure_storage: Arc<dyn SecureStorage>) -> Result<Self> {
        Wallet::init_logging();

        log::info!("Instantiating Wallet by loading the active Profile from storage");

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
        Wallet::init_logging();

        log::info!(
            "Instantiating Wallet by loading the Profile with ID {} from storage",
            profile_id
        );

        Self::new_load_profile_with_id(profile_id, WalletClientStorage::new(secure_storage))
    }
}

#[cfg(test)]
impl Wallet {
    pub(crate) fn ephemeral(profile: Profile) -> (Self, Arc<EphemeralSecureStorage>) {
        let storage = EphemeralSecureStorage::new();
        (
            Self::by_importing_profile(profile, storage.clone()),
            storage,
        )
    }
}
#[cfg(test)]
impl HasPlaceholder for Wallet {
    fn placeholder() -> Self {
        Self::ephemeral(Profile::placeholder()).0
    }

    fn placeholder_other() -> Self {
        Self::ephemeral(Profile::placeholder_other()).0
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
        let value = self.profile
            .try_write()
            .map(mutate)
            .expect("Implementing Wallet clients should not read and write Profile from Wallet from multiple threads.");

        self.save_existing_profile()
            .expect("Failed to save Profile to secure storage.");

        value
    }

    pub(crate) fn try_write<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> Result<R>,
    {
        let res = self
            .profile
            .try_write()
            .map_err(|_| CommonError::UnableToAcquireWriteLockForProfile)
            .and_then(mutate)?;

        self.save_existing_profile()?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_toolkit_json::models::transaction::header;

    use crate::prelude::*;
    #[test]
    fn read_header() {
        let wallet = Wallet::placeholder();
        wallet.read(|p| assert_eq!(p.header, Profile::placeholder().header))
    }

    #[test]
    fn take_snapshot() {
        let wallet = Wallet::placeholder();
        assert_eq!(wallet.profile(), Profile::placeholder())
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn by_loading_profile_with_id() {
        let profile = Profile::placeholder();
        let secure_storage = EphemeralSecureStorage::new();
        let data = serde_json::to_vec(&profile).unwrap();
        assert!(secure_storage
            .save_data(
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile.id(),
                },
                data,
            )
            .is_ok());
        assert_eq!(
            secure_storage.load_data(SecureStorageKey::ActiveProfileID),
            Ok(None)
        ); // we dont have any ActiveID yet.
        let wallet =
            Wallet::by_loading_profile_with_id(profile.id(), secure_storage.clone()).unwrap();
        assert_eq!(wallet.profile(), profile);

        // Assert an ActiveProfileID has been saved.
        assert_eq!(
            secure_storage.load_data(SecureStorageKey::ActiveProfileID),
            Ok(Some(serde_json::to_vec(&profile.id()).unwrap()))
        );
    }

    #[test]
    fn by_loading_profile() {
        let profile = Profile::placeholder();
        let secure_storage = EphemeralSecureStorage::new();
        let active_profile_id_data = serde_json::to_vec(&profile.id()).unwrap();
        assert!(secure_storage
            .save_data(SecureStorageKey::ActiveProfileID, active_profile_id_data)
            .is_ok());
        let data = serde_json::to_vec(&profile).unwrap();
        assert!(secure_storage
            .save_data(
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile.id(),
                },
                data,
            )
            .is_ok());
        let wallet = Wallet::by_loading_profile(secure_storage).unwrap();
        assert_eq!(wallet.profile(), profile);
    }

    #[test]
    fn snapshot_json() {
        let profile = Profile::placeholder();
        let secure_storage = EphemeralSecureStorage::new();
        let wallet = Wallet::by_importing_profile(profile.clone(), secure_storage);
        let expected_json = serde_json::to_string(&profile).unwrap();
        assert_eq!(wallet.json_snapshot(), expected_json);
    }

    #[test]
    fn by_creating_new_profile_and_secrets_with_entropy() {
        let secure_storage = EphemeralSecureStorage::new();
        let wallet =
            Wallet::by_creating_new_profile_and_secrets_with_entropy(
                Vec::from_iter([0xff; 32]),
                WalletClientModel::Unknown,
                "Test".to_string(),
                secure_storage.clone(),
            )
            .unwrap();
        let mnemonic_json = secure_storage
            .load_data(SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: wallet.profile().bdfs().id,
            })
            .unwrap()
            .unwrap();
        let mwp = serde_json::from_slice::<MnemonicWithPassphrase>(&mnemonic_json).unwrap();
        assert_eq!(mwp.mnemonic.phrase(), "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote");

        let active_id_data =
            secure_storage
                .load_data(SecureStorageKey::ActiveProfileID)
                .unwrap()
                .unwrap();

        let active_id = serde_json::from_slice::<ProfileID>(&active_id_data).unwrap();
        assert_eq!(active_id, wallet.profile().id());
    }
}

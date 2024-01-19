use crate::prelude::*;

impl Wallet {
    /// Adds a device factor source to Profile and SecureStorage, this method will only
    /// return `Ok` if both the mnemonic was successfully saved to SecureStorage and the
    /// DeviceFactorSource present in Profile and Profile also successfully updated in
    /// SecureStorage.
    ///
    /// Returns `Err` if it is already present in Profile. It is Wallet Client
    /// dependent if it throws if already present in SecureStorage.
    ///
    /// If saving of `MnemonicWithPassphrase` to SecureStorage succeeds, but adding
    /// `DeviceFactorSource` to Profile/saving of Profile to SecureStorage fails, then
    /// this method will try to remove the newly saved `MnemonicWithPassphrase` from
    /// `SecureStorage`.
    ///
    /// Takes ownership of `PrivateHierarchicalDeterministicFactorSource`
    pub fn add_private_device_factor_source(
        &self,
        private_device_factor_source: PrivateHierarchicalDeterministicFactorSource,
    ) -> Result<()> {
        let id = private_device_factor_source.factor_source.id.clone();

        info!(
            "Save Private DeviceFactorSource to SecureStorage, factor source id: {}",
            &id
        );

        self.wallet_client_storage.save_mnemonic_with_passphrase(
            &private_device_factor_source.mnemonic_with_passphrase,
            &id,
        )?;

        self.add_factor_source(private_device_factor_source.factor_source.into())
            .map_err(|e| {
                error!(
                    "Failed to add Private DeviceFactorSource to SecureStorage, factor source id: {}",
                    id
                );
                _ = self.wallet_client_storage.delete_mnemonic(&id);
                e
            })
    }

    /// Adds `factor_source` to Profile and takes a snapshot of Profile and
    /// updates it in SecureStorage.
    ///
    /// Returns `Err` if `factor_source` is already present in factor source,
    /// or if saving to SecureStorage fails.
    ///
    /// If only saving to SecureStorage fails, the Profile still remains
    /// edited.
    pub fn add_factor_source(&self, factor_source: FactorSource) -> Result<()> {
        self.try_write(|mut p| {
            trace!(
                "About to add FactorSource: {}, to list of factor sources: {}",
                &factor_source,
                &p.factor_sources
            );
            if p.factor_sources.append(factor_source.to_owned()).0 {
                debug!("Added FactorSource: {}", &factor_source);
                Ok(())
            } else {
                error!(
                    "FactorSource not added, already present: {}",
                    &factor_source
                );
                Err(CommonError::Unknown)
            }
        })
        .map_err(
            |_| CommonError::UnableToSaveFactorSourceToProfile(factor_source.factor_source_id())
        )
    }

    /// Loads a `MnemonicWithPassphrase` with the `id` of `device_factor_source`,
    /// from SecureStorage, and returns a `PrivateHierarchicalDeterministicFactorSource`
    /// built from both.
    ///
    /// Useful for when you will want to sign transactions or derive public keys for
    /// creation of new entities.
    ///
    /// Returns `Err` if loading or decoding of `MnemonicWithPassphrase` from
    /// SecureStorage fails.
    pub fn load_private_device_factor_source(
        &self,
        device_factor_source: &DeviceFactorSource,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        info!(
            "Load Private DeviceFactorSource from SecureStorage, factor source id: {}",
            &device_factor_source.id
        );
        self.wallet_client_storage
            .load_mnemonic_with_passphrase(&device_factor_source.id)
            .map(|mwp| {
                PrivateHierarchicalDeterministicFactorSource::new(mwp, device_factor_source.clone())
            })
            .log_info("Successfully loaded Private DeviceFactorSource from SecureStorage")
    }

    /// Loads a `MnemonicWithPassphrase` with the `id` of `device_factor_source`,
    /// from SecureStorage, and returns a `PrivateHierarchicalDeterministicFactorSource`
    /// built from both.
    ///
    /// Useful for when you will want to sign transactions or derive public keys for
    /// creation of new entities.
    ///
    /// Returns `Err` if loading or decoding of `MnemonicWithPassphrase` from
    /// SecureStorage fails.
    pub fn load_private_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        let device_factor_source = self.profile().device_factor_source_by_id(id)?;
        self.load_private_device_factor_source(&device_factor_source)
    }
}

//========
// SET - Account
//========
#[uniffi::export]
impl Wallet {
    /// Creates a new non securified account **WITHOUT** add it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `wallet.add_account(account)`
    pub fn create_new_account(&self, network_id: NetworkID, name: DisplayName) -> Result<Account> {
        let profile = &self.profile();
        let bdfs = profile.bdfs();
        let index = profile.next_derivation_index_for_entity(EntityKind::Accounts, network_id);
        let number_of_accounts_on_network = profile
            .networks
            .get(&network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let appearance_id =
            AppearanceID::from_number_of_accounts_on_network(number_of_accounts_on_network);

        let factor_instance = self
            .load_private_device_factor_source(&bdfs)
            .map(|p| p.derive_account_creation_factor_instance(network_id, index))?;

        let account = Account::new(factor_instance, name, appearance_id);

        Ok(account)
    }

    /// Returns `Ok(())` if the `account` was new and successfully added. If saving failed or if the account was already present in Profile, an
    /// error is returned.
    pub fn add_account(&self, account: Account) -> Result<()> {
        // TODO: clean this up, BAD code. messy, mostly because of (my) bad IdentifiedVec API.
        let network_id = account.network_id.clone();
        let err_exists = CommonError::AccountAlreadyPresent(account.id().clone());
        self.try_write(|mut p| {
            let networks = &mut p.networks;
            if networks.contains_id(&network_id) {
                networks
                    .try_update_with(&network_id, |network| {
                        if network.accounts.append(account.clone()).0 {
                            Ok(network.clone())
                        } else {
                            return Err(err_exists.clone());
                        }
                    })
                    .and_then(|r| if r { Ok(()) } else { Err(err_exists.clone()) })
            } else {
                let network = Network::new(network_id, Accounts::from_iter([account.to_owned()]));
                networks.append(network);
                Ok(())
            }
        })
    }

    /// Create a new Account and adds it to the active Profile.
    pub fn create_new_account_and_add_it_to_profile(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let account = self.create_new_account(network_id, name)?;
        self.add_account(account.clone())?;
        Ok(account)
    }

    /// Updates the display name of account with the provided address, throws an error if the account is unknown to the wallet.
    pub fn change_name_of_account(
        &self,
        address: AccountAddress,
        to: DisplayName,
    ) -> Result<Account> {
        self.write(|mut p| p.update_account(&address, |a| a.display_name = to.to_owned()))
            .ok_or_else(|| CommonError::UnknownAccount)
    }
}

#[cfg(test)]
mod tests {

    use std::{
        borrow::{Borrow, BorrowMut},
        ops::Deref,
        sync::atomic::AtomicBool,
    };

    use crate::prelude::*;
    pub use pretty_assertions::{assert_eq, assert_ne};
    use std::sync::RwLock;

    #[test]
    fn change_display_name_of_accounts() {
        let profile = Profile::placeholder();
        let (wallet, _) = Wallet::ephemeral(profile.clone());
        let account = wallet.read(|p| p.networks[0].accounts[0].clone());
        assert_eq!(account.display_name.value, "Alice");
        assert!(wallet
            .change_name_of_account(account.address, DisplayName::new("Stella").unwrap())
            .is_ok());
        wallet.read(|p| assert_eq!(p.networks[0].accounts[0].display_name.value, "Stella"));

        assert_eq!(
            wallet.change_name_of_account(
                AccountAddress::placeholder_other(),
                DisplayName::new("not used").unwrap()
            ),
            Err(CommonError::UnknownAccount)
        );
    }

    #[test]
    fn load_private_device_factor_source() {
        let private = PrivateHierarchicalDeterministicFactorSource::placeholder();
        let dfs = private.factor_source;
        let profile = Profile::placeholder();
        let (wallet, storage) = Wallet::ephemeral(profile.clone());
        let data = serde_json::to_vec(&private.mnemonic_with_passphrase).unwrap();
        let key = SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: dfs.id.clone(),
        };
        storage.save_data(key.clone(), data.clone()).unwrap();
        assert_eq!(
            wallet
                .load_private_device_factor_source(&dfs)
                .unwrap()
                .mnemonic_with_passphrase,
            MnemonicWithPassphrase::placeholder()
        );
    }

    #[test]
    pub fn add_private_device_factor_source_successful() {
        let profile = Profile::placeholder();
        let new =
            PrivateHierarchicalDeterministicFactorSource::generate_new(WalletClientModel::Unknown);
        let (wallet, storage) = Wallet::ephemeral(profile.clone());
        assert_eq!(
            profile
                .factor_sources
                .contains_id(&new.clone().factor_source.factor_source_id()),
            false
        );
        assert!(wallet.add_private_device_factor_source(new.clone()).is_ok());
        assert!(storage.storage.read().unwrap().contains_key(
            &SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: new.clone().factor_source.id,
            },
        ));
        assert_eq!(
            wallet
                .profile()
                .factor_sources
                .contains_id(&new.clone().factor_source.factor_source_id()),
            true
        );
    }

    #[test]
    pub fn add_private_device_factor_source_ok_storage_when_save_to_profile_fails_then_deleted_from_storage(
    ) {
        let profile = Profile::placeholder();
        let new =
            PrivateHierarchicalDeterministicFactorSource::generate_new(WalletClientModel::Unknown);

        assert_eq!(
            profile
                .factor_sources
                .contains_id(&new.clone().factor_source.factor_source_id()),
            false
        );
        let delete_data_was_called = Arc::new(RwLock::new(Option::<SecureStorageKey>::None));
        #[derive(Debug)]
        struct TestStorage {
            delete_data_was_called: Arc<RwLock<Option<SecureStorageKey>>>,
        }
        impl SecureStorage for TestStorage {
            fn load_data(&self, _key: SecureStorageKey) -> Result<Option<Vec<u8>>> {
                todo!()
            }

            fn save_data(&self, _key: SecureStorageKey, _data: Vec<u8>) -> Result<()> {
                Ok(()) // mnemonic gets saved
            }

            fn delete_data_for_key(&self, key: SecureStorageKey) -> Result<()> {
                let mut delete_data_was_called = self.delete_data_was_called.write().unwrap();
                *delete_data_was_called = Some(key);
                Ok(())
            }
        }
        let storage = Arc::new(TestStorage {
            delete_data_was_called: delete_data_was_called.clone(),
        });
        let wallet = Wallet::by_importing_profile(profile, storage.clone());

        // Acquire write lock, in order to make `wallet.add_private_device_factor_source` fail (because cant have multiple writers).
        let lock = wallet.profile.write().unwrap();

        assert_eq!(
            wallet.add_private_device_factor_source(new.clone()),
            Err(CommonError::UnableToSaveFactorSourceToProfile(
                new.factor_source.factor_source_id()
            ))
        );
        drop(lock);

        assert_eq!(
            wallet
                .profile()
                .factor_sources
                .contains_id(&new.clone().factor_source.factor_source_id()),
            false // should not have been saved.
        );
        assert_eq!(
            delete_data_was_called.read().unwrap().clone().unwrap(),
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: new.clone().factor_source.id
            }
        );
    }

    #[test]
    fn add_factor_source_fails_when_already_exists() {
        let profile = Profile::placeholder();
        let other = PrivateHierarchicalDeterministicFactorSource::placeholder();
        let (wallet, _) = Wallet::ephemeral(profile.clone());
        assert_eq!(
            wallet.add_factor_source(other.factor_source.clone().into()),
            Err(CommonError::UnableToSaveFactorSourceToProfile(
                other.factor_source.factor_source_id()
            ))
        )
    }

    #[test]
    fn load_private_device_factor_source_by_id() {
        let profile = Profile::placeholder();
        let private = PrivateHierarchicalDeterministicFactorSource::placeholder();
        let (wallet, storage) = Wallet::ephemeral(profile.clone());

        let data = serde_json::to_vec(&private.mnemonic_with_passphrase).unwrap();
        let key = SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: private.clone().factor_source.id.clone(),
        };
        assert!(storage.save_data(key.clone(), data).is_ok());

        let loaded = wallet
            .load_private_device_factor_source_by_id(&private.factor_source.id.clone())
            .unwrap();
        assert_eq!(loaded, private);
    }
}

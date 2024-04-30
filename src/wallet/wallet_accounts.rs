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
        let id = private_device_factor_source.factor_source.id;

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
        self.try_update_profile_with(|mut p| {
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
        .map_err(|_| CommonError::UnableToSaveFactorSourceToProfile {
            bad_value: factor_source.factor_source_id(),
        })
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
    pub fn create_new_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let profile = &self.profile();
        let bdfs = profile.bdfs();

        let index = profile
            .next_derivation_index_for_entity(EntityKind::Account, network_id);

        let number_of_accounts_on_network = profile
            .networks
            .get(&network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let appearance_id = AppearanceID::from_number_of_accounts_on_network(
            number_of_accounts_on_network,
        );

        let factor_instance =
            self.load_private_device_factor_source(&bdfs).map(|p| {
                p.derive_entity_creation_factor_instance(network_id, index)
            })?;

        let account = Account::new(factor_instance, name, appearance_id);

        Ok(account)
    }

    /// Returns `Ok(())` if the `account` was new and successfully added. If saving failed or if the account was already present in Profile, an
    /// error is returned.
    pub fn add_account(&self, account: Account) -> Result<()> {
        // TODO: clean this up, BAD code. messy, mostly because of (my) bad IdentifiedVec API.
        let network_id = account.network_id;
        let err_exists = CommonError::AccountAlreadyPresent {
            bad_value: account.id(),
        };
        self.try_update_profile_with(|mut p| {
            let networks = &mut p.networks;
            if networks.contains_id(&network_id) {
                networks
                    .try_update_with(&network_id, |network| {
                        if (*network.accounts).append(account.clone()).0 {
                            Ok(network.clone())
                        } else {
                            Err(err_exists.clone())
                        }
                    })
                    .and_then(
                        |r| if r { Ok(()) } else { Err(err_exists.clone()) },
                    )
            } else {
                let network = ProfileNetwork::new(
                    network_id,
                    Accounts::from_iter([account.to_owned()]),
                    Personas::default(),
                    AuthorizedDapps::default(),
                );
                networks.append(network);
                Ok(())
            }
        })
    }

    /// Create a new Account and adds it to the active Profile.
    pub fn create_and_save_new_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let account = self.create_new_account(network_id, name)?;
        self.add_account(account.clone())?;
        Ok(account)
    }

    /// Updates `account` as a whole, if it exists, else an error is thrown.
    pub fn update_account(&self, to: Account) -> Result<Account> {
        self.update_profile_with(|mut p| {
            p.update_account(&to.address, |a| *a = to.to_owned())
        })
        .ok_or(CommonError::UnknownAccount)
    }

    /// Updates the display name of account with the provided address, throws an error if the account is unknown to the wallet.
    pub fn change_name_of_account(
        &self,
        address: AccountAddress,
        to: DisplayName,
    ) -> Result<Account> {
        self.update_profile_with(|mut p| {
            p.update_account(&address, |a| a.display_name = to.to_owned())
        })
        .ok_or(CommonError::UnknownAccount)
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
        let profile = Profile::sample();
        let (wallet, _) = Wallet::ephemeral(profile.clone());
        let account =
            wallet.access_profile_with(|p| p.networks[0].accounts[0].clone());
        assert_eq!(account.display_name.value, "Alice");
        assert!(wallet
            .change_name_of_account(
                account.address,
                DisplayName::new("Stella").unwrap()
            )
            .is_ok());
        wallet.access_profile_with(|p| {
            assert_eq!(p.networks[0].accounts[0].display_name.value, "Stella")
        });

        assert_eq!(
            wallet.change_name_of_account(
                AccountAddress::sample_other(),
                DisplayName::new("not used").unwrap()
            ),
            Err(CommonError::UnknownAccount)
        );
    }

    #[test]
    fn update_account() {
        let profile = Profile::sample();
        let (wallet, _) = Wallet::ephemeral(profile.clone());
        let mut account =
            wallet.access_profile_with(|p| p.networks[0].accounts[0].clone());
        assert_eq!(account.display_name.value, "Alice");
        account.display_name = DisplayName::new("Stella").unwrap();
        account.appearance_id = AppearanceID::new(7).unwrap();

        // Assert that `Account` returned by method `update_account` is the updated one.
        assert_eq!(
            wallet.update_account(account).unwrap().display_name.value,
            "Stella"
        );

        // Assert account has been updated in `wallet.profile`
        wallet.access_profile_with(|p| {
            let account = &p.networks[0].accounts[0];
            assert_eq!(account.display_name.value, "Stella");
            assert_eq!(account.appearance_id.value, 7);
        });
    }

    #[test]
    fn load_private_device_factor_source() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let dfs = private.factor_source;
        let profile = Profile::sample();
        let (wallet, storage) = Wallet::ephemeral(profile.clone());
        let data =
            serde_json::to_vec(&private.mnemonic_with_passphrase).unwrap();
        let key = SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: dfs.id,
        };
        storage.save_data(key.clone(), data.clone()).unwrap();
        assert_eq!(
            wallet
                .load_private_device_factor_source(&dfs)
                .unwrap()
                .mnemonic_with_passphrase,
            MnemonicWithPassphrase::sample()
        );
    }

    #[test]
    pub fn add_private_device_factor_source_successful() {
        let profile = Profile::sample();
        let new =
            PrivateHierarchicalDeterministicFactorSource::generate_new_babylon(
                true,
                WalletClientModel::Unknown,
            );
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
        let profile = Profile::sample();
        let new =
            PrivateHierarchicalDeterministicFactorSource::generate_new_babylon(
                true,
                WalletClientModel::Unknown,
            );

        assert_eq!(
            profile
                .factor_sources
                .contains_id(&new.clone().factor_source.factor_source_id()),
            false
        );
        let delete_data_was_called =
            Arc::new(RwLock::new(Option::<SecureStorageKey>::None));
        #[derive(Debug)]
        struct TestStorage {
            delete_data_was_called: Arc<RwLock<Option<SecureStorageKey>>>,
        }
        impl SecureStorage for TestStorage {
            fn load_data(
                &self,
                _key: SecureStorageKey,
            ) -> Result<Option<Vec<u8>>> {
                unreachable!()
            }

            fn save_data(
                &self,
                _key: SecureStorageKey,
                _data: Vec<u8>,
            ) -> Result<()> {
                Ok(()) // mnemonic gets saved
            }

            fn delete_data_for_key(&self, key: SecureStorageKey) -> Result<()> {
                let mut delete_data_was_called =
                    self.delete_data_was_called.write().unwrap();
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
            Err(CommonError::UnableToSaveFactorSourceToProfile {
                bad_value: new.factor_source.factor_source_id()
            })
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
        let profile = Profile::sample();
        let other = PrivateHierarchicalDeterministicFactorSource::sample();
        let (wallet, _) = Wallet::ephemeral(profile.clone());
        assert_eq!(
            wallet.add_factor_source(other.factor_source.clone().into()),
            Err(CommonError::UnableToSaveFactorSourceToProfile {
                bad_value: other.factor_source.factor_source_id()
            })
        )
    }

    #[test]
    fn load_private_device_factor_source_by_id() {
        let profile = Profile::sample();
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let (wallet, storage) = Wallet::ephemeral(profile.clone());

        let data =
            serde_json::to_vec(&private.mnemonic_with_passphrase).unwrap();
        let key = SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: private.clone().factor_source.id,
        };
        assert!(storage.save_data(key.clone(), data).is_ok());

        let loaded = wallet
            .load_private_device_factor_source_by_id(
                &private.factor_source.id.clone(),
            )
            .unwrap();
        assert_eq!(loaded, private);
    }

    #[test]
    #[ignore]
    fn generate_huge_profile_with_super_many_accounts() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();

        let (wallet, storage) = Wallet::ephemeral(Profile::new(
            private.clone().factor_source,
            "Test",
        ));

        let data =
            serde_json::to_vec(&private.mnemonic_with_passphrase).unwrap();
        let key = SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: private.clone().factor_source.id,
        };
        assert!(storage.save_data(key.clone(), data).is_ok());

        let network_id = NetworkID::Mainnet;

        let n = 100;
        (0..n).for_each(|index| {
            let account_name =
                DisplayName::new(format!("Account {index}")).unwrap();
            let _ = wallet
                .create_and_save_new_account(network_id, account_name.clone())
                .unwrap();
        });

        let profile = wallet.profile();
        assert_eq!(profile.networks.first().unwrap().accounts.len(), n);
        let profile_json = profile.to_json_bytes();

        fs::write(
            concat!(env!("FIXTURES_VECTOR"), "big_profile_100_accounts.json"),
            profile_json,
        )
        .expect("Unable to write file");
    }

    // Profile `init_profile`'s BDFS MUST eq `PrivateHierarchicalDeterministicFactorSource::sample()`
    fn test_new_account<F, G>(
        init_profile: Profile,
        also_save: bool,
        assert_before: F,
        assert_after: G,
    ) where
        F: Fn(Profile),
        G: Fn(Account, Profile),
    {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        assert_eq!(
            init_profile.bdfs().factor_source_id(),
            private.clone().factor_source.factor_source_id()
        );

        let (wallet, storage) = Wallet::ephemeral(init_profile);
        assert_before(wallet.profile());

        let data =
            serde_json::to_vec(&private.mnemonic_with_passphrase).unwrap();
        let key = SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: private.clone().factor_source.id,
        };
        assert!(storage.save_data(key.clone(), data).is_ok());

        let account_name = DisplayName::new("Test").unwrap();
        let network_id = NetworkID::Mainnet;
        let account = if also_save {
            wallet.create_and_save_new_account(network_id, account_name.clone())
        } else {
            wallet.create_new_account(network_id, account_name.clone())
        }
        .unwrap();

        assert_eq!(account.display_name, account_name);
        assert_eq!(account.network_id, network_id);

        assert_after(account, wallet.profile());
    }

    fn test_create_new_account_first_success<F>(also_save: bool, assert_last: F)
    where
        F: Fn(Account, Profile),
    {
        test_new_account(
            Profile::new(
                PrivateHierarchicalDeterministicFactorSource::sample()
                    .factor_source,
                "Test",
            ),
            also_save,
            |p| {
                assert_eq!(p.networks.len(), 0); // no accounts yet, no networks even
            },
            |a, q| {
                assert_eq!(
                    a.address.address(),
                    "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8"
                );
                assert_eq!(a.appearance_id, AppearanceID::new(0).unwrap()); // using `0` since first.

                assert_last(a, q);
            },
        );
    }

    #[test]
    fn create_new_account_first_success() {
        test_create_new_account_first_success(false, |_, q| {
            assert_eq!(q.networks.len(), 0);
        });

        test_create_new_account_first_success(true, |a, q| {
            assert_eq!(q.networks.len(), 1);
            assert_eq!(q.networks[0].accounts[0], a);
        })
    }

    fn test_create_new_account_not_first_success<F>(
        also_save: bool,
        assert_last: F,
    ) where
        F: Fn(Account, Profile),
    {
        test_new_account(
            Profile::sample(),
            also_save,
            |p| {
                assert_eq!(p.networks[0].accounts.len(), 2);
            },
            |a, q| {
                assert_eq!(
                    a.address.address(),
                    "account_rdx12xvg2sssh0rpca6e8xyqv5vf4nqu928083yzf0fdrnvjdz2pvc000x" // pretty cool address! Random!
                );
                assert_eq!(a.appearance_id, AppearanceID::new(2).unwrap());

                assert_last(a, q);
            },
        );
    }

    #[test]
    fn create_new_account_not_first_success() {
        test_create_new_account_not_first_success(false, |_, q| {
            // Account SHOULD NOT yet have been saved into Profile, so number of accounts should still be 2
            assert_eq!(q.networks[0].accounts.len(), 2);
        });

        test_create_new_account_not_first_success(true, |a, q| {
            assert_eq!(q.networks[0].accounts.len(), 3);
            assert_eq!(q.networks[0].accounts[2], a);
        })
    }
}

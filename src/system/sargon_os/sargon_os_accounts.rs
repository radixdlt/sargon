use std::sync::RwLockWriteGuard;

use crate::prelude::*;

// ==================
// Create Unsaved Account(s)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    pub fn accounts_on_current_network(&self) -> Accounts {
        self.profile_holder.accounts_on_current_network()
    }

    /// Returns the non-hidden accounts on the current network as `AccountForDisplay`
    pub fn accounts_for_display_on_current_network(
        &self,
    ) -> AccountsForDisplay {
        self.profile_holder
            .accounts_for_display_on_current_network()
    }

    /// Looks up the account by account address, returns Err if the account is
    /// unknown, will return a hidden account if queried for.
    pub fn account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<Account> {
        self.profile_holder.account_by_address(address)
    }

    /// Creates a new unsaved mainnet account named "Unnamed {N}", where `N` is the
    /// index of the next account for the BDFS.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_account(
        &self,
    ) -> Result<Account> {
        self.create_unsaved_account(
            NetworkID::Mainnet,
            DisplayName::new("Unnamed").unwrap(),
        )
        .await
    }

    /// Uses `create_unsaved_account` specifying `NetworkID::Mainnet`.
    pub async fn create_unsaved_mainnet_account(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_unsaved_account(NetworkID::Mainnet, name).await
    }

    /// Creates a new non securified account **WITHOUT** adding it to Profile,
    /// using the *main* "Babylon" `DeviceFactorSource` and the "next" index for
    /// this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `os.add_account(account)`.
    pub async fn create_unsaved_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let profile = self.profile();

        let (factor_source_id, account) = profile
            .create_unsaved_account(network_id, name, async move |fs| {
                self.load_private_device_factor_source(&fs).await
            })
            .await?;

        // Change of `last_used_on` of FactorSource
        self.update_last_used_of_factor_source(factor_source_id)
            .await?;

        Ok(account)
    }

    /// Create a new mainnet Account named "Unnamed" and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_unnamed_mainnet_account(
        &self,
    ) -> Result<Account> {
        self.create_and_save_new_mainnet_account(
            DisplayName::new("Unnamed").unwrap(),
        )
        .await
    }

    /// Create a new mainnet Account and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_mainnet_account(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_and_save_new_account(NetworkID::Mainnet, name)
            .await
    }

    /// Create a new Account and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        debug!("Creating account.");
        let account = self.create_unsaved_account(network_id, name).await?;
        debug!("Created account, now saving it to profile.");
        self.add_account(account.clone()).await?;
        info!(
            "Created account and saved new account into profile, address: {}",
            account.address
        );
        Ok(account)
    }

    /// The account names will be `<name_prefix> <index>`
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn batch_create_many_accounts_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<()> {
        debug!("Batch creating #{} accounts.", count);
        let accounts = self
            .batch_create_unsaved_accounts(network_id, count, name_prefix)
            .await?;
        debug!("Created #{} accounts, now saving them to profile.", count);
        self.add_accounts(accounts).await?;
        info!(
            "Created account and saved #{} new accounts into profile",
            count
        );
        Ok(())
    }

    /// Creates many new non securified accounts **WITHOUT** add them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add them to Profile, call `add_accounts(accounts)`
    ///
    /// # Emits Event
    /// Emits `Event::FactorSourceUpdated { id: FactorSourceID }` since the date in
    /// `factor_source.common.last_used` is updated.
    pub async fn batch_create_unsaved_accounts(
        &self,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<Accounts> {
        let profile = self.profile();

        let (factor_source_id, accounts) = profile
            .create_unsaved_accounts(
                network_id,
                count,
                |idx| {
                    DisplayName::new(format!("{} {}", name_prefix, idx))
                        .expect("Should not use a long name_prefix")
                },
                async move |fs| {
                    self.load_private_device_factor_source(&fs).await
                },
            )
            .await?;

        // Change of `last_used_on` of FactorSource
        self.update_last_used_of_factor_source(factor_source_id)
            .await?;

        Ok(accounts)
    }
}

// ==================
// Add (Save) Account(s)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Add the `account` to active profile and **saves** the updated profile to
    /// secure storage.
    ///
    /// Returns `Ok(())` if the `account` was new and successfully added. If
    /// saving failed or if the account was already present in Profile, an
    /// error is returned.
    ///
    /// # Emits Events
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::AccountsAdded { addresses } }`
    pub async fn add_account(&self, account: Account) -> Result<()> {
        let address = account.address;

        debug!("Adding account address: {} to profile", address);

        self.add_accounts_without_emitting_account_added_event(Accounts::just(
            account,
        ))
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::AccountAdded { address },
            ))
            .await;

        Ok(())
    }

    /// Adds the `accounts` to active profile and **saves** the updated profile to
    /// secure storage.
    ///
    /// Returns `Ok(())` if the `accounts` were new and successfully added. If
    /// saving failed or if the accounts were already present in Profile, an
    /// error is returned.
    ///
    /// # Emits Events
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::AccountsAdded { addresses } }`
    pub async fn add_accounts(&self, accounts: Accounts) -> Result<()> {
        let addresses = accounts
            .clone()
            .into_iter()
            .map(|a| a.address)
            .collect_vec();

        self.add_accounts_without_emitting_account_added_event(accounts)
            .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::AccountsAdded { addresses },
            ))
            .await;

        Ok(())
    }
}

// ==================
// Update Account(s)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Updates the account `updated` by mutating current profile and persisting
    /// the change to secure storage. Throws `UnknownAccount` error if the account
    /// is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountUpdated { address } }`
    pub async fn update_account(&self, updated: Account) -> Result<()> {
        self.update_profile_with(|mut p| {
            if p.update_account(&updated.address, |old| *old = updated.clone())
                .is_none()
            {
                Err(CommonError::UnknownAccount)
            } else {
                Ok(())
            }
        })
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::AccountUpdated {
                    address: updated.address,
                },
            ))
            .await;

        Ok(())
    }
}

impl SargonOS {
    /// Adds the `accounts` to active profile and **saves** the updated profile to
    /// secure storage, without emitting `Event::AccountAdded`, but we DO emit
    /// `Event::ProfileSaved`.`
    ///
    /// Returns `Ok(())` if the `accounts` were new and successfully added. If
    /// saving failed or if the accounts were already present in Profile, an
    /// error is returned.
    ///
    /// # Emits
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    async fn add_accounts_without_emitting_account_added_event(
        &self,
        accounts: Accounts,
    ) -> Result<()> {
        if accounts.is_empty() {
            warn!("Tried to add empty accounts...");
            return Ok(());
        }

        let number_of_accounts_to_add = accounts.len();

        let network_id = accounts
            .assert_elements_on_same_network()?
            .expect("Should have handled empty accounts case already.");

        debug!("Adding #{} accounts to Profile Network with ID: {} - or creating a Profile Network if it does not exist", number_of_accounts_to_add, network_id);

        self.update_profile_with(|mut p| {
            let networks = &mut p.networks;

            if networks.contains_id(network_id) {
                debug!("Profile already contained network to add #{} account(s) to, network_id: {}", number_of_accounts_to_add, network_id);
                networks
                    .try_try_update_with(&network_id, |network| {
                        let count_before = network.accounts.len();
                        debug!("Profile Network to add #{} account(s) to contains #{} accounts (before adding).", number_of_accounts_to_add, count_before);
                        network.accounts.extend(accounts.clone());
                        let count_after = network.accounts.len();
                        debug!("Profile Network now contains: #{} accounts", count_after);
                        if network.accounts.len() == count_before + number_of_accounts_to_add {
                            Ok(())
                        } else {
                            Err(CommonError::UnableToAddAllAccountsDuplicatesFound)
                        }
                    })
            } else {
                debug!("No Profile Network exists with ID {}, creating it...", network_id);
                let network = ProfileNetwork::new(
                    network_id,
                    accounts.clone(),
                    Personas::default(),
                    AuthorizedDapps::default(),
                );
                networks.append(network);
                Ok(())
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_first_add_account() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.add_account(Account::sample()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.profile().networks[0].accounts.len(), 1);
    }

    #[actix_rt::test]
    async fn test_content_hint_is_updated_when_accounts_are_added() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.add_account(Account::sample()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile()
                .header
                .content_hint
                .number_of_accounts_on_all_networks_in_total,
            1
        );
        assert_eq!(os.profile().header.content_hint.number_of_networks, 1);
    }

    #[actix_rt::test]
    async fn test_first_create_unsaved_account() {
        // ARRANGE
        let os = SUT::fast_boot_bdfs(MnemonicWithPassphrase::sample()).await;

        // ACT
        let unsaved_account = os
            .with_timeout(|x| {
                x.create_unsaved_mainnet_account(
                    DisplayName::new("Alice").unwrap(),
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(unsaved_account, Account::sample());
        assert_eq!(os.profile().networks[0].accounts.len(), 0); // not added
    }

    #[actix_rt::test]
    async fn test_create_unsaved_account_twice_yield_same_accounts() {
        // ARRANGE
        let os = SUT::fast_boot_bdfs(MnemonicWithPassphrase::sample()).await;

        // ACT
        let first = os
            .with_timeout(|x| x.create_unsaved_unnamed_mainnet_account())
            .await
            .unwrap();

        let second = os
            .with_timeout(|x| x.create_unsaved_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert_eq!(first, second);
    }

    #[actix_rt::test]
    async fn test_first_create_and_add_account_is_added() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.profile().networks[0].accounts, Accounts::just(account));
    }

    #[actix_rt::test]
    async fn test_first_create_and_add_account_has_index_0() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            account
                .security_state
                .into_unsecured()
                .unwrap()
                .transaction_signing
                .derivation_path()
                .hd_path()
                .components
                .last()
                .unwrap()
                .index(),
            0
        );
    }

    #[actix_rt::test]
    async fn test_second_create_and_add_account_has_index_1() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let _ = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        let second = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            second
                .security_state
                .into_unsecured()
                .unwrap()
                .transaction_signing
                .derivation_path()
                .hd_path()
                .components
                .last()
                .unwrap()
                .index(),
            1
        );
    }

    #[actix_rt::test]
    async fn batch_create_account_then_n_accounts_are_saved_and_have_indices_0_through_n(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n: u32 = 3;
        os.with_timeout(|x| {
            x.batch_create_many_accounts_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let indices = os.profile().networks[0]
            .accounts
            .iter()
            .map(|x| {
                x.security_state
                    .into_unsecured()
                    .unwrap()
                    .transaction_signing
                    .derivation_path()
                    .hd_path()
                    .components
                    .last()
                    .unwrap()
                    .index()
            })
            .collect_vec();
        assert_eq!(indices, (0u32..n).collect_vec());
    }

    #[actix_rt::test]
    async fn test_batch_create_and_add_account_n_has_names_with_index_appended_to_prefix(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n: u32 = 3;
        os.with_timeout(|x| {
            x.batch_create_many_accounts_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let names = os.profile().networks[0]
            .accounts
            .iter()
            .map(|x| x.display_name.value.clone())
            .collect_vec();

        assert_eq!(
            names,
            ["test 0", "test 1", "test 2"]
                .into_iter()
                .map(|x| x.to_owned())
                .collect_vec()
        );
    }

    #[actix_rt::test]
    async fn batch_create_account_then_n_accounts_are_saved_and_have_appearance_id_0_through_max(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let n = AppearanceID::all().len() as u32 * 2;
        os.with_timeout(|x| {
            x.batch_create_many_accounts_then_save_once(
                n as u16,
                NetworkID::Mainnet,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        let appearance_ids = os.profile().networks[0]
            .accounts
            .iter()
            .map(|x| x.appearance_id)
            .collect_vec();

        assert_eq!(
            appearance_ids,
            [AppearanceID::all(), AppearanceID::all()].concat()
        );
    }

    #[actix_rt::test]
    async fn batch_create_account_unsaved_are_not_saved() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| {
            x.batch_create_unsaved_accounts(
                NetworkID::Mainnet,
                3,
                "test".to_owned(),
            )
        })
        .await
        .unwrap();

        // ASSERT
        assert!(os.profile().networks[0].accounts.is_empty())
    }

    #[actix_rt::test]
    async fn test_create_unsaved_account_emits_factor_source_updated() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        // ACT
        os.with_timeout(|x| x.create_unsaved_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert!(event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::FactorSourceUpdated));
    }

    impl DisplayName {
        fn random() -> Self {
            Self::new(format!(
                "random-{}",
                id().to_string().drain(0..20).collect::<String>()
            ))
            .unwrap()
        }
    }

    #[actix_rt::test]
    async fn update_account_updates_in_memory_profile() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let mut account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        account.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_account(account.clone()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.profile().networks[0].accounts[0], account.clone())
    }

    #[actix_rt::test]
    async fn update_account_updates_saved_profile() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let mut account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        account.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_account(account.clone()))
            .await
            .unwrap();

        // ASSERT
        let saved_profile = os
            .with_timeout(|x| x.secure_storage.load_active_profile())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(saved_profile.networks[0].accounts[0], account.clone())
    }

    #[actix_rt::test]
    async fn test_update_account_emits() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        let mut account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        account.display_name = DisplayName::random();
        os.with_timeout(|x| x.update_account(account.clone()))
            .await
            .unwrap();

        // ASSERT
        assert!(event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::AccountUpdated));
    }

    #[actix_rt::test]
    async fn update_account_unknown_accounts_throws() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let result = os
            .with_timeout(|x| x.update_account(Account::sample()))
            .await;

        // ASSERT
        assert_eq!(result, Err(CommonError::UnknownAccount))
    }

    #[actix_rt::test]
    async fn add_accounts_empty_is_ok() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let result = os.with_timeout(|x| x.add_accounts(Accounts::new())).await;

        // ASSERT
        assert!(result.is_ok())
    }

    #[actix_rt::test]
    async fn add_accounts_duplicates_throws() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let account = Account::sample();
        os.with_timeout(|x| x.add_account(account.clone()))
            .await
            .unwrap();

        // ACT
        let result = os
            .with_timeout(|x| x.add_accounts(Accounts::just(account.clone())))
            .await;

        // ASSERT
        assert_eq!(
            result,
            Err(CommonError::UnableToAddAllAccountsDuplicatesFound)
        )
    }

    #[actix_rt::test]
    async fn test_accounts_on_current_network_empty() {
        let os = SUT::fast_boot().await;
        assert_eq!(os.accounts_on_current_network(), Accounts::new());
    }

    #[actix_rt::test]
    async fn test_accounts_on_current_network_non_empty() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.accounts_on_current_network(), Accounts::just(account));
    }

    #[actix_rt::test]
    async fn test_accounts_on_current_network_empty_when_switched_network() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let _ = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ACT
        let _ = os
            .with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.accounts_on_current_network(), Accounts::new());
    }

    #[actix_rt::test]
    async fn test_accounts_for_display_on_current_network() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.accounts_for_display_on_current_network(),
            AccountsForDisplay::just(AccountForDisplay::from(account))
        );
    }

    #[actix_rt::test]
    async fn test_account_by_address_exists() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let account = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.account_by_address(account.address), Ok(account));
    }

    #[actix_rt::test]
    async fn test_account_by_address_not_exists() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        // so that we have at least one network (with one account)
        let _ = os
            .with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.account_by_address(AccountAddress::sample_mainnet()),
            Err(CommonError::UnknownAccount)
        );
    }
}

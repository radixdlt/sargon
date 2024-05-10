use std::sync::RwLockWriteGuard;

use crate::prelude::*;

// ==================
// Create Unsaved Account(s)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Creates a new non securified account **WITHOUT** add it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `wallet.add_account(account)`
    pub async fn create_unsaved_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let profile = self.profile();
        profile
            .create_unsaved_account(network_id, name, async move |fs| {
                self.load_private_device_factor_source(&fs).await
            })
            .await
    }

    /// Create a new Account and adds it to the active Profile.
    pub async fn create_and_save_new_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        info!("Creating account.");
        let account = self.create_unsaved_account(network_id, name).await?;
        info!("Created account, now saving it to profile.");
        self.add_account(account.clone()).await?;
        info!(
            "Created account and saved new account into profile, address: {}",
            account.address
        );
        Ok(account)
    }

    /// The account names will be `<name_prefix> <index>`
    pub async fn batch_create_many_accounts_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<()> {
        info!("Batch creating #{} accounts.", count);
        let accounts = self
            .batch_create_unsaved_accounts(network_id, count, name_prefix)
            .await?;
        info!("Created #{} accounts, now saving them to profile.", count);
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
    pub async fn batch_create_unsaved_accounts(
        &self,
        network_id: NetworkID,
        count: u16,
        name_prefix: String,
    ) -> Result<Accounts> {
        let profile = self.profile();
        profile
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
            .await
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
    pub async fn add_account(&self, account: Account) -> Result<()> {
        let address = account.address;

        debug!("Adding account address: {} to profile", address);
        self.add_accounts_without_emitting_event(Accounts::just(account))
            .await?;

        self.profile_holder.access_profile_with(|p| {
            let accounts_on_network = p
                .networks
                .get_id(&address.network_id())
                .unwrap()
                .accounts
                .len();
            debug!(
                "Added account address: {} to profile, contains: #{}",
                address, accounts_on_network
            );
        });

        self.event_bus
            .emit(EventNotification::new(Event::ProfileChanged {
                change: ProfileChange::AddedAccount { address },
            }))
            .await;

        Ok(())
    }

    /// Adds the `accounts` to active profile and **saves** the updated profile to
    /// secure storage.
    ///
    /// Returns `Ok(())` if the `accounts` were new and successfully added. If
    /// saving failed or if the accounts were already present in Profile, an
    /// error is returned.
    pub async fn add_accounts(&self, accounts: Accounts) -> Result<()> {
        let addresses = accounts
            .clone()
            .into_iter()
            .map(|a| a.address)
            .collect_vec();

        self.add_accounts_without_emitting_event(accounts).await?;

        self.event_bus
            .emit(EventNotification::new(Event::ProfileChanged {
                change: ProfileChange::AddedAccounts { addresses },
            }))
            .await;

        Ok(())
    }
}

// ==================
// Update Account(s)
// ==================
#[uniffi::export]
impl SargonOS {
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
        .await
    }
}

impl SargonOS {
    /// Adds the `accounts` to active profile and **saves** the updated profile to
    /// secure storage.
    ///
    /// Returns `Ok(())` if the `accounts` were new and successfully added. If
    /// saving failed or if the accounts were already present in Profile, an
    /// error is returned.
    async fn add_accounts_without_emitting_event(
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

            if networks.contains_id(&network_id) {
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
impl Drivers {
    pub fn test() -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        )
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
    async fn test_add_account() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.add_account(Account::sample()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.profile().networks[0].accounts.len(), 1);
    }
}

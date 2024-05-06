use std::sync::RwLockWriteGuard;

use crate::prelude::*;

trait OnSameNetworkValidating: Clone + IntoIterator<Item = Self::Element> {
    type Element: IsNetworkAware;

    fn is_empty(&self) -> bool;

    fn assert_elements_not_empty_and_on_same_network(
        &self,
    ) -> Result<NetworkID> {
        self.assert_elements_on_same_network()
            .and_then(|x| x.ok_or(CommonError::ExpectedNonEmptyCollection))
    }

    fn assert_elements_on_same_network(&self) -> Result<Option<NetworkID>> {
        if self.is_empty() {
            return Ok(None);
        }
        let network_id = self.clone().into_iter().last().unwrap().network_id();
        self.clone()
            .into_iter()
            .map(|e| {
                if e.network_id() == network_id {
                    Ok(())
                } else {
                    Err(CommonError::NetworkDiscrepancy {
                        expected: network_id,
                        actual: e.network_id(),
                    })
                }
            })
            .collect::<Result<()>>()?;

        Ok(Some(network_id))
    }
}
impl OnSameNetworkValidating for Accounts {
    type Element = Account;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[uniffi::export]
impl SargonOS {
    pub fn has_any_network(&self) -> bool {
        self.profile_holder
            .access_profile_with(|p| !p.networks.is_empty())
    }

    pub fn profile(&self) -> Profile {
        self.profile_holder.profile()
    }

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
                .get(&address.network_id())
                .unwrap()
                .accounts
                .len();
            debug!(
                "Added account address: {} to profile, contains: #{}",
                address, accounts_on_network
            );
        });

        self.clients
            .event_bus
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

        self.clients
            .event_bus
            .emit(EventNotification::new(Event::ProfileChanged {
                change: ProfileChange::AddedAccounts { addresses },
            }))
            .await;

        Ok(())
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
    /// Updates and **saves** profile to secure storage, after
    /// mutating it with `mutate`.
    pub(crate) async fn update_profile_with<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> Result<R>,
    {
        let res = self.profile_holder.update_profile_with(mutate)?;
        self.save_existing_profile().await?;
        Ok(res)
    }

    pub(crate) async fn save_existing_profile(&self) -> Result<()> {
        self.save_profile(&self.profile()).await
    }

    pub(crate) async fn save_profile(&self, profile: &Profile) -> Result<()> {
        let secure_storage = &self.clients.secure_storage;

        secure_storage
            .save(
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile.header.id,
                },
                profile,
            )
            .await?;

        self.clients
            .event_bus
            .emit(EventNotification::new(Event::ProfileChanged {
                change: ProfileChange::UnspecifiedChange,
            }))
            .await;

        Ok(())
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

        let number_of_accounts_to_add = (&accounts).len();

        let network_id = accounts
            .assert_elements_on_same_network()?
            .expect("Should have handled empty accounts case already.");

        debug!("Adding #{} accounts to Profile Network with ID: {} - or creating a Profile Network if it does not exist", number_of_accounts_to_add, network_id);

        self.update_profile_with(|mut p| {
            let networks = &mut p.networks;

            if networks.contains_id(&network_id) {
                debug!("Profile already contained network to add #{} account(s) to, network_id: {}", number_of_accounts_to_add, network_id);
                networks
                // TODO: clean this up, BAD code. messy, mostly because of (my) bad IdentifiedVec API.
                    .try_update_with(&network_id, |network| {
                        let count_before = network.accounts.len();
                        debug!("Profile Network to add #{} account(s) to contains #{} accounts (before adding).", number_of_accounts_to_add, count_before);
                        (*network.accounts).append_other(accounts.clone());
                        let count_after = network.accounts.len();
                        debug!("Profile Network now contains: #{} accounts", count_after);
                        if network.accounts.len() == count_before + number_of_accounts_to_add {
                            Ok(network.clone())
                        } else {
                            Err(CommonError::UnableToAddAllAccountsDuplicatesFound)
                        }
                    })
                .and_then(
                    |r| {
                        if r {
                            Ok(())
                        } else {
                            Err(CommonError::UnableToAddAllAccountsDuplicatesFound)
                        }
                    },
                )
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_rt::time::timeout;
//     use std::time::Duration;

//     const MAX: Duration = Duration::from_millis(10);

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = SargonOS;

//     #[actix_rt::test]
//     async fn test_add_account() {
//         let sut = SargonOS::sample();
//         let req = sut.add_account(Account::sample());
//         let result = timeout(MAX, req).await.unwrap();
//     }
// }

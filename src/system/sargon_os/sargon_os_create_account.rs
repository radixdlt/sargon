use std::sync::RwLockWriteGuard;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
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

    /// Returns `Ok(())` if the `account` was new and successfully added. If
    /// saving failed or if the account was already present in Profile, an
    /// error is returned.
    pub async fn add_account(&self, account: Account) -> Result<()> {
        // TODO: clean this up, BAD code. messy, mostly because of (my) bad IdentifiedVec API.
        let network_id = account.network_id;
        let err_exists = CommonError::AccountAlreadyPresent {
            bad_value: account.id(),
        };
        self.update_profile_with(
            ProfileChange::AddedAccount {
                address: account.address,
            },
            |mut p| {
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
                        .and_then(|r| {
                            if r {
                                Ok(())
                            } else {
                                Err(err_exists.clone())
                            }
                        })
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
            },
        )
        .await
    }

    /// Create a new Account and adds it to the active Profile.
    pub async fn create_and_save_new_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let account = self.create_unsaved_account(network_id, name).await?;
        self.add_account(account.clone()).await?;
        Ok(account)
    }
}

impl SargonOS {
    // /// Updates `account` as a whole, if it exists, else an error is thrown.
    // pub fn update_account(&self, to: Account) -> Result<Account> {
    //     self.update_profile_with(|mut p| {
    //         p.update_account(&to.address, |a| *a = to.to_owned())
    //     })
    //     .ok_or(CommonError::UnknownAccount)
    // }

    pub(crate) async fn update_profile_with<F, R>(
        &self,
        event: ProfileChange,
        mutate: F,
    ) -> Result<R>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> Result<R>,
    {
        let res = self.profile_holder.update_profile_with(mutate)?;
        self.save_existing_profile().await?;
        self.clients
            .event_bus
            .emit(Event::ProfileChanged { change: event })
            .await;
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
            .await
    }
}

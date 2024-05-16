use crate::prelude::*;
use std::{
    borrow::Borrow,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug, uniffi::Object)]
#[allow(dead_code)]
pub struct ProfileHolder {
    // This is pub(crate) for testing purposes only, i.e. causing the RwLock to be poisoned.
    pub(crate) profile: RwLock<Profile>,
}

impl ProfileHolder {
    pub fn new(profile: Profile) -> Self {
        Self {
            profile: RwLock::new(profile),
        }
    }
}

impl ProfileHolder {
    /// Clone the profile and return it.
    pub(super) fn profile(&self) -> Profile {
        self.access_profile_with(|p| p.clone())
    }

    pub fn current_network_id(&self) -> NetworkID {
        self.access_profile_with(|p| p.current_network_id())
    }

    pub fn current_gateway(&self) -> Gateway {
        self.access_profile_with(|p| p.current_gateway().clone())
    }

    pub fn current_network(&self) -> ProfileNetwork {
        self.access_profile_with(|p| p.current_network().clone())
    }

    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    pub fn accounts_on_current_network(&self) -> Accounts {
        self.access_profile_with(|p| p.accounts_on_current_network())
    }

    /// Returns the non-hidden accounts on the current network as `AccountForDisplay`
    pub fn accounts_for_display_on_current_network(
        &self,
    ) -> AccountsForDisplay {
        self.access_profile_with(|p| {
            p.accounts_for_display_on_current_network()
        })
    }

    /// Looks up the account by account address, returns Err if the account is
    /// unknown, will return a hidden account if queried for.
    pub fn account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<Account> {
        self.access_profile_with(|p| p.account_by_address(address))
    }

    pub(super) fn access_profile_with<T, F>(&self, access: F) -> T
    where
        F: Fn(RwLockReadGuard<'_, Profile>) -> T,
    {
        self.profile
            .try_read()
            .map(access)
            .expect("Implementing Wallet clients should not read and write Profile from Wallet from multiple threads.")
    }

    /// Sets the profile held by this ProfileHolder to `profile`.
    pub(super) fn replace_profile_with(&self, profile: Profile) -> Result<()> {
        let mut lock = self
            .profile
            .try_write()
            .map_err(|_| CommonError::UnableToAcquireWriteLockForProfile)?;

        *lock = profile;
        Ok(())
    }

    /// Updates the in-memory profile held by this `ProfileHolder`, you might
    /// wanna also persist the change in the `SargonOS` by saving it to secure
    /// storage.
    pub(super) fn update_profile_with<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> Result<R>,
    {
        self.profile
            .try_write()
            .map_err(|_| CommonError::UnableToAcquireWriteLockForProfile)
            .and_then(mutate)
    }
}

use std::sync::RwLockWriteGuard;

use crate::prelude::*;

// ==================
// Create Unsaved Account(s)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    pub fn accounts_on_current_network(&self) -> Result<Accounts> {
        map_result_from_internal(self.wrapped.accounts_on_current_network())
    }

    /// Returns the non-hidden accounts on the current network as `AccountForDisplay`
    pub fn accounts_for_display_on_current_network(
        &self,
    ) -> Result<AccountsForDisplay> {
        map_result_from_internal(self.wrapped
            .accounts_for_display_on_current_network())
    }

    /// Looks up the account by account address, returns Err if the account is
    /// unknown, will return a hidden account if queried for.
    pub fn account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<Account> {
        map_result_from_internal(self.wrapped.account_by_address(address.into()))
    }

    /// Creates a new unsaved mainnet account named "Unnamed {N}", where `N` is the
    /// index of the next account for the BDFS.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_account(
        &self,
    ) -> Result<Account> {
        map_result_from_internal(self.wrapped.create_unsaved_unnamed_mainnet_account().await)
    }

    /// Uses `create_unsaved_account` specifying `NetworkID::Mainnet`.
    pub async fn create_unsaved_mainnet_account(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        map_result_from_internal(self.wrapped.create_unsaved_mainnet_account(name.into()).await)
    }

    /// Creates a new non securified account **WITHOUT** adding it to Profile,
    /// using the *main* "Babylon" `DeviceFactorSource` and the "next" index for
    /// this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `os.add_account(account)`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage, since the `last_used_on` date
    /// of the factor source has been updated.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn create_unsaved_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        map_result_from_internal(self.wrapped.create_unsaved_account(network_id.into(), name.into()).await)
    }

    /// Create a new mainnet Account named "Unnamed" and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_unnamed_mainnet_account(
        &self,
    ) -> Result<Account> {
        map_result_from_internal(self.wrapped.create_and_save_new_unnamed_mainnet_account()
            .await)
    }

    /// Create a new mainnet Account and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_mainnet_account(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        map_result_from_internal(self.wrapped
            .create_and_save_new_mainnet_account(name.into())
            .await)
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
        map_result_from_internal(self.wrapped
            .create_and_save_new_account(network_id.into(), name.into())
            .await)
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
        map_result_from_internal(self.wrapped.batch_create_many_accounts_then_save_once(
            count,
            network_id.into(),
            name_prefix,
        ).await)
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
        self.wrapped.batch_create_unsaved_accounts(
            network_id.into(),
            count,
            name_prefix,
        ).await.map_result()
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
        self.wrapped.add_account(account.into()).await.map_result()
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
        self.wrapped.add_accounts(accounts.into_internal_vec()).await.map_result()
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
        self.wrapped.update_account(updated.into()).await.map_result()
    }
}
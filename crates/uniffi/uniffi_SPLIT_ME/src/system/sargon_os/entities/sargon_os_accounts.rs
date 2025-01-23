use crate::prelude::*;

// ==================
// Create Unsaved Account(s)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    pub fn accounts_on_current_network(&self) -> Result<Vec<Account>> {
        self.wrapped
            .accounts_on_current_network()
            .into_iter_result()
    }

    /// Returns the non-hidden accounts on the current network as `AccountForDisplay`
    pub fn accounts_for_display_on_current_network(
        &self,
    ) -> Result<Vec<AccountForDisplay>> {
        self.wrapped
            .accounts_for_display_on_current_network()
            .into_iter_result()
    }

    /// Looks up the account by account address, returns Err if the account is
    /// unknown, will return a hidden account if queried for.
    pub fn account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<Account> {
        self.wrapped
            .account_by_address(address.into_internal())
            .into_result()
    }

    /// Creates a new unsaved mainnet account named "Unnamed {N}", where `N` is the
    /// index of the next account for the BDFS.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_account_with_main_bdfs(
        &self,
    ) -> Result<Account> {
        self.wrapped
            .create_unsaved_unnamed_mainnet_account_with_main_bdfs()
            .await
            .into_result()
    }

    /// Uses `create_unsaved_account` specifying `NetworkID::Mainnet` using main BDFS.
    pub async fn create_unsaved_mainnet_account_with_main_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        self.wrapped
            .create_unsaved_mainnet_account_with_main_bdfs(name.into_internal())
            .await
            .into_result()
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
    pub async fn create_unsaved_account_with_main_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        self.wrapped
            .create_unsaved_account_with_main_bdfs(
                network_id.into_internal(),
                name.into_internal(),
            )
            .await
            .into_result()
    }

    /// Create a new mainnet Account named "Unnamed" using main BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_unnamed_mainnet_account_with_main_bdfs(
        &self,
    ) -> Result<Account> {
        self.wrapped
            .create_and_save_new_unnamed_mainnet_account_with_main_bdfs()
            .await
            .into_result()
    }

    /// Create a new mainnet Account using the main BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_mainnet_account_with_main_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        self.wrapped
            .create_and_save_new_mainnet_account_with_main_bdfs(
                name.into_internal(),
            )
            .await
            .into_result()
    }

    /// Create a new Account and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        self.wrapped
            .create_and_save_new_account_with_factor_source(
                factor_source.into_internal(),
                network_id.into_internal(),
                name.into_internal(),
            )
            .await
            .into_result()
    }

    /// Create a new Account using main BDFS and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_account_with_main_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        self.wrapped
            .create_and_save_new_account_with_main_bdfs(
                network_id.into_internal(),
                name.into_internal(),
            )
            .await
            .into_result()
    }

    /// Creates account using main BDFS.
    /// The account names will be `<name_prefix> <index>`
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn batch_create_many_accounts_with_main_bdfs_then_save_once(
        &self,
        count: u16,
        network_id: NetworkID,
        name_prefix: String,
    ) -> Result<()> {
        self.wrapped
            .batch_create_many_accounts_with_main_bdfs_then_save_once(
                count,
                network_id.into_internal(),
                name_prefix,
            )
            .await
            .map(|_| {})
            .into_result()
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
    ) -> Result<Vec<Account>> {
        self.wrapped
            .batch_create_unsaved_accounts_with_main_bdfs_consuming_factor_instances(
                network_id.into_internal(),
                count,
                name_prefix,
            )
            .await
            .into_iter_result()
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
        self.wrapped
            .add_account(account.into_internal())
            .await
            .into_result()
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
    pub async fn add_accounts(&self, accounts: Vec<Account>) -> Result<()> {
        self.wrapped
            .add_accounts(accounts.into_internal())
            .await
            .into_result()
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
        self.wrapped
            .update_account(updated.into_internal())
            .await
            .into_result()
    }

    /// Updates the profile by marking the account with `account_address` as hidden.
    pub async fn mark_account_as_hidden(
        &self,
        account_address: AccountAddress,
    ) -> Result<()> {
        self.wrapped
            .mark_account_as_hidden(account_address.into_internal())
            .await
            .into_result()
    }

    /// Updates the profile by marking the account with `account_address` as tombstoned.
    pub async fn mark_account_as_tombstoned(
        &self,
        account_address: AccountAddress,
    ) -> Result<()> {
        self.wrapped
            .mark_account_as_tombstoned(account_address.into_internal())
            .await
            .into_result()
    }
}

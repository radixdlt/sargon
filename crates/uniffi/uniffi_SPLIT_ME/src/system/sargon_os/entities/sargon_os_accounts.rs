use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
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

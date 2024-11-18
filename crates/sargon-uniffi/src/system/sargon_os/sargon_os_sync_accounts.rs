use crate::prelude::*;

// ==================
// Sync Profile Accounts with status on ledger
// ==================
impl SargonOS {
    /// Checks all active accounts in current network on ledger, if any of them are deleted.
    /// Any deleted account is marked as tombstoned in profile.
    ///
    /// Returns true if any account became tombstoned.
    pub async fn sync_accounts_deleted_on_ledger(&self) -> Result<bool> {
        self.wrapped
            .sync_accounts_deleted_on_ledger()
            .await
            .into_result()
    }

    /// Queries all `account_addresses` on ledger and checks reports which one is deleted.
    ///
    /// Returns an array of the account addresses along with a `bool` being true if that account
    /// is deleted
    pub async fn check_accounts_deleted_on_ledger(
        &self,
        network_id: NetworkID,
        account_addresses: impl IntoIterator<Item = AccountAddress>,
    ) -> Result<Vec<(AccountAddress, bool)>> {
        let result = self
            .wrapped
            .check_accounts_deleted_on_ledger(
                network_id.into_internal(),
                account_addresses.into_iter().map(|a| a.into_internal()),
            )
            .await
            .map_err(|internal| CommonError::from(internal))?;

        Ok(result
            .iter()
            .cloned()
            .map(|(internal_account_address, is_deleted)| {
                (AccountAddress::from(internal_account_address), is_deleted)
            })
            .collect_vec())
    }
}
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
}

use sargon::OsSyncEntitiesStateOnLedger;

use crate::prelude::*;

// ==================
// Sync Profile Accounts with status on ledger
// ==================
#[uniffi::export]
impl SargonOS {
    /// Syncs entities in profile with on ledger state.
    /// Returns a summary report of the actions performed.
    ///
    /// Checks performed on entities are:
    /// 1. Checks if active accounts on profile are deleted on ledger.
    ///    Action => to mark them as tombstoned
    /// 2. Checks if entities with provisional shield are securified on ledger.
    ///    Action => to mark them as securified
    /// 3. more checks will be developed...
    async fn wip_sync_entities_state_on_ledger(
        &self,
    ) -> Result<EntitySyncOutcome> {
        self.wrapped
            .wip_sync_entities_state_on_ledger()
            .await
            .into_result()
    }

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
        account_addresses: Vec<AccountAddress>,
    ) -> Result<HashMap<AccountAddress, bool>> {
        let result = self
            .wrapped
            .check_accounts_deleted_on_ledger(
                network_id.into_internal(),
                account_addresses
                    .iter()
                    .map(|a| a.into_internal())
                    .collect(),
            )
            .await
            .map_err(CommonError::from)?;

        Ok(result
            .into_iter()
            .map(|(account_address, is_deleted)| {
                (account_address.into(), is_deleted)
            })
            .collect())
    }
}

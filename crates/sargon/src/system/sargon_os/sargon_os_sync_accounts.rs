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
        let accounts = self.accounts_on_current_network()?;

        let network_id = self.profile_state_holder.current_network_id()?;

        let account_addresses_with_deleted_status = self
            .check_accounts_deleted_on_ledger(
                network_id,
                accounts.iter().map(|a| a.address),
            )
            .await?;

        let account_addresses_to_tombstone =
            account_addresses_with_deleted_status
                .iter()
                .filter_map(|(account_address, is_deleted)| {
                    if *is_deleted {
                        Some(account_address.clone())
                    } else {
                        None
                    }
                })
                .collect_vec();

        let is_any_account_tombstoned =
            !account_addresses_to_tombstone.is_empty();

        if is_any_account_tombstoned {
            self.mark_accounts_as_tombstoned(account_addresses_to_tombstone)
                .await?;
        }

        Ok(is_any_account_tombstoned)
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
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        gateway_client
            .check_account_is_deleted(network_id, account_addresses)
            .await
    }
}

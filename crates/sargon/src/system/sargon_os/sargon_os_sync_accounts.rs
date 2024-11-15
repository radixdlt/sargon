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
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        let active_account_addresses =
            accounts.iter().map(|a| a.address).collect::<Vec<_>>();
        let on_ledger_checks = active_account_addresses
            .iter()
            .map(|address| {
                gateway_client.check_account_is_deleted(address.clone())
            })
            .collect_vec();
        let account_addresses_with_deleted_status =
            try_join_all(on_ledger_checks).await?;

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

        let any_account_tombstoned = !account_addresses_to_tombstone.is_empty();

        if any_account_tombstoned {
            self.mark_accounts_as_tombstoned(account_addresses_to_tombstone)
                .await?;
        }

        Ok(any_account_tombstoned)
    }
}

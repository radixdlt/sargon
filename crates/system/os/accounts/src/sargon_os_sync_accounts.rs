use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsSyncAccountsDeletedOnLedger {
    async fn sync_accounts_deleted_on_ledger(&self) -> Result<bool>;
    async fn check_accounts_deleted_on_ledger(
        &self,
        network_id: NetworkID,
        account_addresses: IndexSet<AccountAddress>,
    ) -> Result<IndexMap<AccountAddress, bool>>;
}

// ==================
// Sync Profile Accounts with status on ledger
// ==================
#[async_trait::async_trait]
impl OsSyncAccountsDeletedOnLedger for SargonOS {
    /// Checks all active accounts in current network on ledger, if any of them are deleted.
    /// Any deleted account is marked as tombstoned in profile.
    ///
    /// Returns true if any account became tombstoned.
    async fn sync_accounts_deleted_on_ledger(&self) -> Result<bool> {
        let accounts = self.accounts_on_current_network()?;

        let network_id = self.current_network_id()?;

        let account_addresses_with_deleted_status = self
            .check_accounts_deleted_on_ledger(
                network_id,
                accounts.iter().map(|a| a.address).collect(),
            )
            .await?;

        let account_addresses_to_tombstone =
            account_addresses_with_deleted_status
                .iter()
                .filter_map(|(account_address, is_deleted)| {
                    if *is_deleted {
                        Some(*account_address)
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
    async fn check_accounts_deleted_on_ledger(
        &self,
        network_id: NetworkID,
        account_addresses: IndexSet<AccountAddress>,
    ) -> Result<IndexMap<AccountAddress, bool>> {
        let gateway_client =
            GatewayClient::new(self.http_client.driver.clone(), network_id);

        gateway_client
            .check_accounts_are_deleted(network_id, account_addresses)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    use radix_common::prelude::ACCOUNT_OWNER_BADGE as SCRYPTO_ACCOUNT_OWNER_BADGE;

    #[actix_rt::test]
    async fn test_sync_accounts_deleted_on_ledger() {
        // ARRANGE
        let account_deleted_on_ledger = Account::sample_mainnet_alice();
        let account_active_on_ledger = Account::sample_mainnet_bob();
        let all_initial_accounts = vec![
            account_deleted_on_ledger.clone(),
            account_active_on_ledger.clone(),
        ];
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            mock_location_response(vec![account_deleted_on_ledger.address]),
            mock_location_response(vec![account_deleted_on_ledger.address]),
        ]);
        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));
        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap();
        os.import_wallet(
            &Profile::with(
                Header::sample(),
                FactorSources::sample(),
                AppPreferences::sample(),
                ProfileNetworks::just(ProfileNetwork::new_with_accounts(
                    NetworkID::Mainnet,
                    all_initial_accounts.clone(),
                )),
            ),
            true,
        )
        .await
        .unwrap();

        // ACT
        // Assert first that accounts in profile are active
        assert_eq!(
            os.accounts_on_current_network().unwrap(),
            Accounts::from_iter(all_initial_accounts.clone())
        );
        let result = os.sync_accounts_deleted_on_ledger().await.unwrap();

        // ASSERT
        // An account is deleted
        assert!(result);
        // Only one account is active, the result is persisted in profile
        assert_eq!(
            os.accounts_on_current_network().unwrap(),
            Accounts::from_iter(vec![account_active_on_ledger])
        );
        // Same request does not respond with any accounts deleted
        assert!(!os.sync_accounts_deleted_on_ledger().await.unwrap())
    }

    #[actix_rt::test]
    async fn test_check_accounts_deleted_on_ledger() {
        // ARRANGE
        let account_address_deleted = AccountAddress::sample_stokenet();
        let account_address_not_deleted =
            AccountAddress::sample_stokenet_other();
        let mock_driver = MockNetworkingDriver::new_with_responses(vec![
            mock_location_response(vec![account_address_deleted]),
        ]);
        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));
        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap();

        // ACT
        let accounts_status = os
            .check_accounts_deleted_on_ledger(
                NetworkID::Stokenet,
                IndexSet::from_iter([
                    account_address_deleted,
                    account_address_not_deleted,
                ]),
            )
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            accounts_status,
            IndexMap::<AccountAddress, bool>::from_iter([
                (account_address_deleted, true),
                (account_address_not_deleted, false),
            ])
        )
    }

    // Addresses passed to this function, will be considered as deleted by the mocked network driver
    fn mock_location_response(
        deleted_account_addresses: Vec<AccountAddress>,
    ) -> MockNetworkingDriverResponse {
        let body = StateNonFungibleLocationResponse {
            ledger_state: LedgerState::sample_stokenet(),
            resource_address: ResourceAddress::new_from_node_id(
                SCRYPTO_ACCOUNT_OWNER_BADGE,
                NetworkID::Stokenet,
            )
            .unwrap(),
            non_fungible_ids: deleted_account_addresses
                .iter()
                .map(|a| {
                    let local_id = NonFungibleLocalId::from(*a);
                    let parent = Address::from(*a);
                    StateNonFungibleLocationResponseItem {
                        non_fungible_id: local_id,
                        is_burned: false,
                        last_updated_at_state_version: 0,
                        owning_vault_address: VaultAddress::sample_stokenet(),
                        owning_vault_parent_ancestor_address: Some(parent),
                        owning_vault_global_ancestor_address: Some(parent),
                    }
                })
                .collect_vec(),
        };

        MockNetworkingDriverResponse::new_success(body)
    }
}

use crate::prelude::*;
use radix_engine_interface::blueprints::account::{
    AccountRemoveAuthorizedDepositorInput as ScryptoAccountRemoveAuthorizedDepositorInput,
    AccountRemoveResourcePreferenceInput as ScryptoAccountRemoveResourcePreferenceInput,
};

#[async_trait::async_trait]
pub trait OsCreateDeleteAccountManifest {
    async fn create_delete_account_manifest(
        &self,
        account_address: AccountAddress,
        recipient_account_address: Option<AccountAddress>,
    ) -> Result<CreateDeleteAccountManifestOutcome>;

    async fn fetch_resource_preferences(
        &self,
        account_address: AccountAddress,
        gateway_client: &GatewayClient,
        ledger_state: LedgerState,
    ) -> Result<Vec<ScryptoAccountRemoveResourcePreferenceInput>>;

    async fn fetch_authorized_depositors(
        &self,
        account_address: AccountAddress,
        gateway_client: &GatewayClient,
        ledger_state: LedgerState,
    ) -> Result<Vec<ScryptoAccountRemoveAuthorizedDepositorInput>>;

    async fn fetch_account_transfers(
        &self,
        account_address: AccountAddress,
        recipient_account_address: Option<AccountAddress>,
        gateway_client: &GatewayClient,
        ledger_state: LedgerState,
    ) -> Result<Option<DeleteAccountTransfers>>;
}

// ==================
// Delete Account (Public)
// ==================
#[async_trait::async_trait]
impl OsCreateDeleteAccountManifest for SargonOS {
    async fn create_delete_account_manifest(
        &self,
        account_address: AccountAddress,
        recipient_account_address: Option<AccountAddress>,
    ) -> Result<CreateDeleteAccountManifestOutcome> {
        let network_id = account_address.network_id();
        let gateway_client = self.gateway_client_with(network_id);

        // Get current ledger state
        let ledger_state = gateway_client.gateway_status().await?.ledger_state;

        // Fetch account transfers
        let account_transfers = self
            .fetch_account_transfers(
                account_address,
                recipient_account_address,
                &gateway_client,
                ledger_state.clone(),
            )
            .await?;

        // Fetch all resource preferences
        let resource_preferences = self
            .fetch_resource_preferences(
                account_address,
                &gateway_client,
                ledger_state.clone(),
            )
            .await?;

        // Fetch all authorized depositors
        let authorized_depositors = self
            .fetch_authorized_depositors(
                account_address,
                &gateway_client,
                ledger_state.clone(),
            )
            .await?;

        // Build Manifest
        let manifest = TransactionManifest::delete_account(
            &account_address,
            account_transfers.clone(),
            resource_preferences,
            authorized_depositors,
        );

        // Build result
        let result = CreateDeleteAccountManifestOutcome::new(
            manifest,
            account_transfers
                .map_or_else(Vec::new, |t| t.non_transferable_resources),
        );
        Ok(result)
    }

    async fn fetch_resource_preferences(
        &self,
        account_address: AccountAddress,
        gateway_client: &GatewayClient,
        ledger_state: LedgerState,
    ) -> Result<Vec<ScryptoAccountRemoveResourcePreferenceInput>> {
        let resource_preferences = gateway_client
            .fetch_all_account_resource_preferences(
                account_address,
                ledger_state.clone().into(),
            )
            .await?
            .into_iter()
            .map(ScryptoAccountRemoveResourcePreferenceInput::from)
            .collect();

        Ok(resource_preferences)
    }

    async fn fetch_authorized_depositors(
        &self,
        account_address: AccountAddress,
        gateway_client: &GatewayClient,
        ledger_state: LedgerState,
    ) -> Result<Vec<ScryptoAccountRemoveAuthorizedDepositorInput>> {
        let authorized_depositors = gateway_client
            .fetch_all_account_authorized_depositors(account_address, ledger_state.into())
            .await?
            .into_iter()
            .map(ScryptoAccountRemoveAuthorizedDepositorInput::try_from)
            .collect::<Result<Vec<ScryptoAccountRemoveAuthorizedDepositorInput>>>()?;

        Ok(authorized_depositors)
    }

    async fn fetch_account_transfers(
        &self,
        account_address: AccountAddress,
        recipient_account_address: Option<AccountAddress>,
        gateway_client: &GatewayClient,
        ledger_state: LedgerState,
    ) -> Result<Option<DeleteAccountTransfers>> {
        // If there is no recipient, there is no transfer to be made.
        let recipient = match recipient_account_address {
            Some(address) => address,
            None => return Ok(None),
        };

        // Get all resources
        let resources = gateway_client
            .fetch_all_resources(account_address, ledger_state.into())
            .await?;

        // Filter transferable resources
        let transferable_resources = gateway_client
            .filter_transferable_resources(resources)
            .await?;

        // Try to build the DeleteAccountTransfers from output and return it.
        let transfers = DeleteAccountTransfers::try_from((
            transferable_resources,
            recipient,
        ))?;
        Ok(Some(transfers))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn manifest() {
        // Test the manifest is correctly built

        // Simulate two empty responses for resource preferences and authorized depositors.
        let os = boot_success(vec![
            gateway_status_response(),
            empty_page_response(),
            empty_page_response(),
        ])
        .await;

        let account_address = AccountAddress::sample();
        let result = os
            .create_delete_account_manifest(account_address, None)
            .await
            .unwrap();

        let expected = TransactionManifest::delete_account(
            &account_address,
            None,
            vec![],
            vec![],
        );

        assert_eq!(result.manifest, expected);
        assert!(result.non_transferable_resources.is_empty());
    }

    /// Boots SargonOS with a mock networking driver that will return the provided responses.
    async fn boot_success(
        responses: Vec<MockNetworkingDriverResponse>,
    ) -> Arc<SargonOS> {
        let mock_driver = MockNetworkingDriver::new_with_responses(responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap()
    }

    /// Creates a mock response for GatewayStatusResponse.
    fn gateway_status_response() -> MockNetworkingDriverResponse {
        MockNetworkingDriverResponse::new_success(GatewayStatusResponse {
            ledger_state: LedgerState::sample(),
        })
    }

    /// Creates a mock response for an empty PageResponse.
    fn empty_page_response() -> MockNetworkingDriverResponse {
        let items: Vec<AccountResourcePreference> = vec![];
        MockNetworkingDriverResponse::new_success(PageResponse::new(
            LedgerState::sample(),
            0,
            None,
            items,
        ))
    }

    #[test]
    fn from_account_authorized_depositor() {
        // Test a ResourceBadge
        let resource_address = ResourceAddress::sample();
        let depositor =
            AccountAuthorizedDepositor::ResourceBadge { resource_address };
        let result = ResourceOrNonFungible::try_from(depositor)
            .expect("Expected a result");

        assert_eq!(
            result,
            ResourceOrNonFungible::Resource {
                value: resource_address
            }
        );

        // Test a FungibleBadge with an integer id
        let nft_collection_address =
            ResourceAddress::sample_stokenet_nft_abandon();
        let depositor = AccountAuthorizedDepositor::NonFungibleBadge {
            resource_address: nft_collection_address,
            non_fungible_id: "#1#".to_string(),
        };

        let result = ResourceOrNonFungible::try_from(depositor)
            .expect("Expected a result");

        assert_eq!(
            result,
            ResourceOrNonFungible::NonFungible {
                value: NonFungibleGlobalId::new_unchecked(
                    nft_collection_address,
                    NonFungibleLocalId::integer(1)
                )
            }
        );

        // Test a FungibleBadge with a String id
        let depositor = AccountAuthorizedDepositor::NonFungibleBadge {
            resource_address: nft_collection_address,
            non_fungible_id: "<Member_237>".to_string(),
        };

        let result = ResourceOrNonFungible::try_from(depositor)
            .expect("Expected a result");

        assert_eq!(
            result,
            ResourceOrNonFungible::NonFungible {
                value: NonFungibleGlobalId::new_unchecked(
                    nft_collection_address,
                    NonFungibleLocalId::string("Member_237").unwrap()
                )
            }
        );

        // Test a FungibleBadge with a Bytes id
        let depositor = AccountAuthorizedDepositor::NonFungibleBadge {
            resource_address: nft_collection_address,
            non_fungible_id: "[deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead]".to_string(),
        };

        let result = ResourceOrNonFungible::try_from(depositor)
            .expect("Expected a result");

        assert_eq!(
            result,
            ResourceOrNonFungible::NonFungible {
                value: NonFungibleGlobalId::new_unchecked(
                    nft_collection_address,
                    NonFungibleLocalId::bytes(Exactly32Bytes::sample_dead())
                        .unwrap()
                )
            }
        );

        // Test a FungibleBadge with Ruid
        let depositor = AccountAuthorizedDepositor::NonFungibleBadge {
            resource_address: nft_collection_address,
            non_fungible_id: "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}".to_string(),
        };

        let result = ResourceOrNonFungible::try_from(depositor)
            .expect("Expected a result");

        assert_eq!(result, ResourceOrNonFungible::NonFungible {
            value: NonFungibleGlobalId::new_unchecked(
                nft_collection_address,
                NonFungibleLocalId::ruid(
                    hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
                ).unwrap()
            )
        });

        // Test a FungibleBadge with an invalid id
        let depositor = AccountAuthorizedDepositor::NonFungibleBadge {
            resource_address: nft_collection_address,
            non_fungible_id: "invalid".to_string(),
        };

        let result = ResourceOrNonFungible::try_from(depositor)
            .expect_err("Expected an error");

        assert_eq!(result, CommonError::InvalidNonFungibleLocalIDString);
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::prelude::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    /// Boots SargonOS with a mock networking driver that will return the provided responses.
    async fn boot_success(
        responses: Vec<MockNetworkingDriverResponse>,
    ) -> Arc<SargonOS> {
        let mock_driver = MockNetworkingDriver::new_with_responses(responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap()
    }

    /// Creates a mock response for GatewayStatusResponse.
    fn gateway_status_response() -> MockNetworkingDriverResponse {
        MockNetworkingDriverResponse::new_success(GatewayStatusResponse {
            ledger_state: LedgerState::sample(),
        })
    }

    /// Creates a mock response for an empty resource preferences PageResponse.
    fn empty_resource_preferences_response() -> MockNetworkingDriverResponse {
        let items: Vec<AccountResourcePreference> = vec![];
        MockNetworkingDriverResponse::new_success(PageResponse::new(
            LedgerState::sample(),
            0,
            None,
            items,
        ))
    }

    /// Creates a mock response for an empty authorized depositors PageResponse.
    fn empty_authorized_depositors_response() -> MockNetworkingDriverResponse {
        let items: Vec<AccountAuthorizedDepositor> = vec![];
        MockNetworkingDriverResponse::new_success(PageResponse::new(
            LedgerState::sample(),
            0,
            None,
            items,
        ))
    }

    fn account_entity_details_response(
        account_address: AccountAddress,
        resource_address: ResourceAddress,
    ) -> MockNetworkingDriverResponse {
        let non_fungible_item = NonFungibleResourcesCollectionItem::Global(
            NonFungibleResourcesCollectionItemGloballyAggregated::new(
                resource_address,
                1,
            ),
        );
        let non_fungible_collection = NonFungibleResourcesCollection::new(
            None,
            None,
            vec![non_fungible_item],
        );
        let item = StateEntityDetailsResponseItem::new(
            account_address.into(),
            None,
            non_fungible_collection,
            EntityMetadataCollection::empty(),
            None,
        );
        MockNetworkingDriverResponse::new_success(
            StateEntityDetailsResponse::new(LedgerState::sample(), vec![item]),
        )
    }

    fn non_transferable_resource_details_response(
        resource_address: ResourceAddress,
    ) -> MockNetworkingDriverResponse {
        let details =
            StateEntityDetailsResponseItemDetails::NonFungibleResource(
                StateEntityDetailsResponseNonFungibleResourceDetails::new(
                    ComponentEntityRoleAssignments::sample_deny_all(),
                ),
            );
        let item = StateEntityDetailsResponseItem::new(
            resource_address.into(),
            None,
            None,
            EntityMetadataCollection::empty(),
            details,
        );
        MockNetworkingDriverResponse::new_success(
            StateEntityDetailsResponse::new(LedgerState::sample(), vec![item]),
        )
    }

    #[actix_rt::test]
    async fn empty_account() {
        // This test verifies that we can correctly create a manifest for the deletion of a virtual account.
        let os = boot_success(vec![
            gateway_status_response(),
            empty_resource_preferences_response(),
            empty_authorized_depositors_response(),
        ])
        .await;

        // Empty/virtual account
        let account_address = AccountAddress::try_from_bech32("account_tdx_2_12856d8p4llz8rs97hx964c5mqyewgwz620awgzuwxhfqgxvyd8n9a7").unwrap();

        let result = os
            .create_delete_account_manifest(account_address, None)
            .await;

        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn account_with_non_transferable_assets() {
        // This test verifies that we can correctly create a manifest for the deletion of an account with non-transferable assets.
        // Account with RadQuest Hero Badge
        let account_address = AccountAddress::try_from_bech32("account_tdx_2_129ty2n42x82qe6unxxpq8m8avjqaff54zfpfpepaaqn2tapqwnc0vw").unwrap();
        let non_transferable_resource: ResourceAddress =
            "resource_tdx_2_1nt72qwswkjkaayfwgyy0d2un8wvpjlq2dg5lq54382wlmf6yly8vz5"
                .parse()
                .unwrap();

        let os = boot_success(vec![
            gateway_status_response(),
            account_entity_details_response(
                account_address,
                non_transferable_resource,
            ),
            non_transferable_resource_details_response(
                non_transferable_resource,
            ),
            empty_resource_preferences_response(),
            empty_authorized_depositors_response(),
        ])
        .await;

        let recipient_address = AccountAddress::sample_stokenet();

        let result = os
            .create_delete_account_manifest(
                account_address,
                recipient_address.into(),
            )
            .await
            .unwrap();

        assert_eq!(
            result.non_transferable_resources,
            vec![
            non_transferable_resource, // RadQuest Hero Badge
        ]
        );
    }
}

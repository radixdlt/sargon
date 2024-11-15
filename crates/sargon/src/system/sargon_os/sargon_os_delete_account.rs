use crate::prelude::*;
use radix_engine_interface::blueprints::account::{
    AccountRemoveAuthorizedDepositorInput as ScryptoAccountRemoveAuthorizedDepositorInput,
    AccountRemoveResourcePreferenceInput as ScryptoAccountRemoveResourcePreferenceInput,
};
use std::future::Future;

// ==================
// Delete Account (Public)
// ==================
impl SargonOS {
    pub async fn create_delete_account_manifest(
        &self,
        account_address: AccountAddress,
        recipient_account_address: Option<AccountAddress>,
    ) -> Result<TransactionManifest> {
        let network_id = account_address.network_id();
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

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
            account_transfers,
            resource_preferences,
            authorized_depositors,
        );

        Ok(manifest)
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
        let output = gateway_client
            .fetch_all_resources(account_address, ledger_state.into())
            .await?;

        // Try to build the DeleteAccountTransfers from output and return it.
        let transfers = DeleteAccountTransfers::try_from((output, recipient))?;
        Ok(Some(transfers))
    }
}

impl From<AccountResourcePreference>
    for ScryptoAccountRemoveResourcePreferenceInput
{
    fn from(value: AccountResourcePreference) -> Self {
        Self {
            resource_address: value.resource_address.into(),
        }
    }
}

impl TryFrom<AccountAuthorizedDepositor>
    for ScryptoAccountRemoveAuthorizedDepositorInput
{
    type Error = CommonError;
    fn try_from(value: AccountAuthorizedDepositor) -> Result<Self> {
        let resource_or_non_fungible = ResourceOrNonFungible::try_from(value)?;
        Ok(resource_or_non_fungible.into())
    }
}

impl TryFrom<AccountAuthorizedDepositor> for ResourceOrNonFungible {
    type Error = CommonError;
    fn try_from(value: AccountAuthorizedDepositor) -> Result<Self> {
        match value {
            AccountAuthorizedDepositor::ResourceBadge { resource_address } => {
                Ok(Self::Resource {
                    value: resource_address,
                })
            }
            AccountAuthorizedDepositor::NonFungibleBadge {
                resource_address,
                non_fungible_id,
            } => {
                if let Ok(non_fungible_id) =
                    NonFungibleLocalId::from_str(&non_fungible_id)
                {
                    Ok(Self::NonFungible {
                        value: NonFungibleGlobalId::new_unchecked(
                            resource_address,
                            non_fungible_id,
                        ),
                    })
                } else {
                    Err(CommonError::InvalidNonFungibleLocalIDString)
                }
            }
        }
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

        assert_eq!(result, expected);
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
                    NonFungibleLocalId::string("Member_237")
                        .unwrap()
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

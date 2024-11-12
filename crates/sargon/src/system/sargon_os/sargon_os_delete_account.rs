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
    ) -> Result<TransactionManifest> {
        let network_id = account_address.network_id();
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        let resource_preferences = self
            .load_all_pages(account_address, |request| {
                gateway_client.account_resource_preferences(request)
            })
            .await?
            .into_iter()
            .map(ScryptoAccountRemoveResourcePreferenceInput::from)
            .collect();

        let authorized_depositors = self
            .load_all_pages(account_address, |request| {
                gateway_client.account_authorized_depositors(request)
            })
            .await?
            .into_iter()
            .map(ScryptoAccountRemoveAuthorizedDepositorInput::try_from)
            .collect::<Result<Vec<ScryptoAccountRemoveAuthorizedDepositorInput>>>()?;

        let manifest = TransactionManifest::delete_account(
            &account_address,
            resource_preferences,
            authorized_depositors,
        );

        Ok(manifest)
    }
}

// ==================
// Load all pages (Internal)
// ==================
impl SargonOS {
    /// Load all pages of a paginated API call that takes an `AccountPageRequest` and returns a `PageResponse`.
    async fn load_all_pages<T, F, Fut>(
        &self,
        account_address: AccountAddress,
        api_call: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(AccountPageRequest) -> Fut,
        Fut: Future<Output = Result<PageResponse<T>>>,
    {
        let mut items: Vec<T> = Vec::new();
        let mut more_to_load = true;
        let mut cursor: Option<String> = None;
        while more_to_load {
            let request = AccountPageRequest::new(
                account_address,
                cursor.clone(),
                GATEWAY_PAGE_REQUEST_LIMIT,
            );
            let response = api_call(request).await?;
            items.extend(response.items);
            cursor = response.next_cursor;
            more_to_load = cursor.is_some();
        }

        Ok(items)
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
                    return Err(CommonError::InvalidNonFungibleLocalIDString);
                }
            }
        }
    }
}

#[cfg(test)]
mod try_from_tests {
    use crate::prelude::*;

    #[test]
    fn account_authorized_depositor() {
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
                    NonFungibleLocalId::string("Member_237".to_string())
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

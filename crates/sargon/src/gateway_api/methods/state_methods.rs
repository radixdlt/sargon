use crate::prelude::*;

impl GatewayClient {
    /// Fetched the XRD balance of account of `address`, returns `None` if
    /// it has no balance.
    pub async fn xrd_balance_of_account(
        &self,
        address: AccountAddress,
    ) -> Result<Option<Decimal192>> {
        let response: StateEntityDetailsResponse = self
            .state_entity_details(StateEntityDetailsRequest::new(
                vec![address.into()],
                None,
                None,
            ))
            .await?;

        let Some(response_item) = response
            .items
            .into_iter()
            .find(|x| x.address == address.into())
        else {
            return Ok(None);
        };

        let fungible_resources = response_item
            .fungible_resources
            .expect("Never None for Account");

        let xrd_address = ResourceAddress::xrd_on_network(address.network_id());

        let Some(xrd_resource_collection_item) = fungible_resources
            .items
            .into_iter()
            .find(|x| x.resource_address() == xrd_address)
        else {
            return Ok(None);
        };

        let xrd_resource = xrd_resource_collection_item
            .as_global()
            .expect("Global is default");

        Ok(Some(xrd_resource.amount))
    }

    /// Fetched the XRD balance of account of `address`, returns `0` if
    /// it has no balance.
    pub async fn xrd_balance_of_account_or_zero(
        &self,
        address: AccountAddress,
    ) -> Result<Decimal192> {
        self.xrd_balance_of_account(address)
            .await
            .map(|x| x.unwrap_or(Decimal192::zero()))
    }
}

impl GatewayClient {
    /// Fetches the metadata for the given entity.
    pub async fn fetch_entity_metadata(
        &self,
        address: Address,
        explicit_metadata: Vec<MetadataKey>,
    ) -> Result<EntityMetadataCollection> {
        let response = self
            .state_entity_details(StateEntityDetailsRequest::address_metadata(
                address,
                explicit_metadata,
            ))
            .await?;

        let Some(response_item) =
            response.items.into_iter().find(|x| x.address == address)
        else {
            return Err(CommonError::EntityNotFound);
        };

        Ok(response_item.metadata)
    }

    /// Fetches the metadata for the given dapp.
    pub async fn fetch_dapp_metadata(
        &self,
        address: DappDefinitionAddress,
    ) -> Result<EntityMetadataCollection> {
        self.fetch_entity_metadata(address.into(), dapp_metadata_keys())
            .await
    }
}

impl GatewayClient {
    /// Fetches all the account's resource preferences.
    pub async fn fetch_all_account_resource_preferences(
        &self,
        account_address: AccountAddress,
        ledger_state_selector: LedgerStateSelector,
    ) -> Result<Vec<AccountResourcePreference>> {
        self.load_all_pages(
            None,
            ledger_state_selector,
            |cursor, ledger_state_selector| {
                let request = AccountResourcePreferencesRequest::new(
                    account_address,
                    ledger_state_selector,
                    cursor,
                    GATEWAY_PAGE_REQUEST_LIMIT,
                );
                self.account_resource_preferences(request)
            },
        )
        .await
    }

    /// Fetches all the account's authorized depositors.
    pub async fn fetch_all_account_authorized_depositors(
        &self,
        account_address: AccountAddress,
        ledger_state_selector: LedgerStateSelector,
    ) -> Result<Vec<AccountAuthorizedDepositor>> {
        self.load_all_pages(
            None,
            ledger_state_selector,
            |cursor, ledger_state_selector| {
                let request = AccountAuthorizedDepositorsRequest::new(
                    account_address,
                    ledger_state_selector,
                    cursor,
                    GATEWAY_PAGE_REQUEST_LIMIT,
                );
                self.account_authorized_depositors(request)
            },
        )
        .await
    }
}

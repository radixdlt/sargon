use crate::prelude::*;

impl GatewayClient {
    /// Get Entity Details
    ///
    /// Returns detailed information for collection of entities. Aggregate resources globally by default.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/StateEntityDetails
    pub async fn state_entity_details(
        &self,
        request: StateEntityDetailsRequest,
    ) -> Result<StateEntityDetailsResponse> {
        self.post(Self::PATH_STATE_ENTITY_DETAILS, request, res_id)
            .await
    }

    /// Get page of Global Entity Fungible Resource Balances
    ///
    /// Returns the total amount of each fungible resource owned by a given global entity.
    /// Result can be aggregated globally or per vault.
    /// The returned response is in a paginated format, ordered by the resource's first appearance on the ledger.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/EntityFungiblesPage
    pub async fn state_entity_page_fungibles(
        &self,
        request: StateEntityPageFungiblesRequest,
    ) -> Result<PageResponse<FungibleResourcesCollectionItem>> {
        self.post(Self::PATH_STATE_ENTITY_PAGE_FUNGIBLES, request, res_id)
            .await
    }

    /// Get page of Global Entity Non-Fungible Resource Balances
    ///
    /// Returns the total amount of each non_fungible resource owned by a given global entity.
    /// Result can be aggregated globally or per vault.
    /// The returned response is in a paginated format, ordered by the resource's first appearance on the ledger.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/EntityNonFungiblesPage
    pub async fn state_entity_page_non_fungibles(
        &self,
        request: StateEntityPageNonFungiblesRequest,
    ) -> Result<PageResponse<NonFungibleResourcesCollectionItem>> {
        self.post(Self::PATH_STATE_ENTITY_PAGE_NON_FUNGIBLES, request, res_id)
            .await
    }

    /// Get page of Global Entity Non-Fungible Resource Vaults
    /// Returns vaults for non fungible resource owned by a given global entity.
    /// The returned response is in a paginated format, ordered by the resource's first
    /// appearance on the ledger.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/EntityNonFungibleResourceVaultPage
    #[allow(dead_code)]
    pub async fn state_entity_page_non_fungible_vaults(
        &self,
        request: StateEntityPageNonFungibleVaultsRequest,
    ) -> Result<
        PageResponse<
            NonFungibleResourcesCollectionItemVaultAggregatedVaultItem,
        >,
    > {
        self.post(
            Self::PATH_STATE_ENTITY_PAGE_NON_FUNGIBLE_VAULTS,
            request,
            res_id,
        )
        .await
    }

    /// Get page of Non-Fungibles in Vault
    ///
    /// Returns all non_fungible IDs of a given non_fungible resource owned by a given entity.
    /// The returned response is in a paginated format, ordered by the resource's first appearance
    /// on the ledger.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/EntityNonFungibleIdsPage
    #[allow(dead_code)]
    pub async fn state_entity_page_non_fungible_vault_ids(
        &self,
        request: StateEntityPageNonFungibleVaultIdsRequest,
    ) -> Result<PageResponse<NonFungibleLocalId>> {
        self.post(
            Self::PATH_STATE_ENTITY_PAGE_NON_FUNGIBLE_VAULT_IDS,
            request,
            res_id,
        )
        .await
    }
}

impl GatewayClient {
    /// Get Account resource preferences page
    ///
    /// Returns paginable collection of resource preference rules for given account.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/AccountResourcePreferencesPage
    pub async fn account_page_resource_preferences(
        &self,
        request: AccountPageResourcePreferencesRequest,
    ) -> Result<PageResponse<AccountResourcePreference>> {
        self.post(
            Self::PATH_ACCOUNT_PAGE_RESOURCE_PREFERENCES,
            request,
            res_id,
        )
        .await
    }

    /// Get Account authorized depositors
    ///
    /// Returns paginable collection of authorized depositors for given account.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/AccountAuthorizedDepositorsPage
    pub async fn account_page_authorized_depositors(
        &self,
        request: AccountPageAuthorizedDepositorsRequest,
    ) -> Result<PageResponse<AccountAuthorizedDepositor>> {
        self.post(
            Self::PATH_ACCOUNT_PAGE_AUTHORIZED_DEPOSITORS,
            request,
            res_id,
        )
        .await
    }
}

impl GatewayClient {
    /// Get Non-Fungible Location
    ///
    /// Returns location of a given non-fungible ID.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/NonFungibleLocation
    pub async fn state_non_fungible_location(
        &self,
        request: StateNonFungibleLocationRequest,
    ) -> Result<StateNonFungibleLocationResponse> {
        self.post(Self::PATH_STATE_NON_FUNGIBLE_LOCATION, request, res_id)
            .await
    }

    pub async fn state_non_fungible_data(
        &self,
        request: StateNonFungibleDataRequest,
    ) -> Result<StateNonFungibleDataResponse> {
        self.post(Self::PATH_STATE_NON_FUNGIBLE_DATA, request, res_id)
            .await
    }
}

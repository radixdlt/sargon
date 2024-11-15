use crate::prelude::*;

impl GatewayClient {
    /// Get Entity Details
    ///
    /// Returns detailed information for collection of entities. Aggregate resources globally by default.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/StateEntityDetails
    pub(crate) async fn state_entity_details(
        &self,
        request: StateEntityDetailsRequest,
    ) -> Result<StateEntityDetailsResponse> {
        self.post("state/entity/details", request, res_id).await
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
    pub(crate) async fn state_entity_page_fungibles(
        &self,
        request: StateEntityPageFungiblesRequest,
    ) -> Result<PageResponse<FungibleResourcesCollectionItem>> {
        self.post("state/entity/page/fungibles/", request, res_id)
            .await
    }

    /// Get page of Global Entity Non-Fungible Resource Balances
    ///
    /// Returns the total amount of each non-fungible resource owned by a given global entity.
    /// Result can be aggregated globally or per vault.
    /// The returned response is in a paginated format, ordered by the resource's first appearance on the ledger.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/EntityNonFungiblesPage
    pub(crate) async fn state_entity_page_non_fungibles(
        &self,
        request: StateEntityPageNonFungiblesRequest,
    ) -> Result<PageResponse<NonFungibleResourcesCollectionItem>> {
        self.post("state/entity/page/non-fungibles/", request, res_id)
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
    pub(crate) async fn state_entity_page_non_fungible_vaults(
        &self,
        request: StateEntityPageNonFungibleVaultsRequest,
    ) -> Result<
        PageResponse<
            NonFungibleResourcesCollectionItemVaultAggregatedVaultItem,
        >,
    > {
        self.post("state/entity/page/non-fungible-vaults/", request, res_id)
            .await
    }

    /// Get page of Non-Fungibles in Vault
    ///
    // Returns all non-fungible IDs of a given non-fungible resource owned by a given entity.
    // The returned response is in a paginated format, ordered by the resource's first appearance
    // on the ledger.
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/EntityNonFungibleIdsPage
    pub(crate) async fn state_entity_page_non_fungible_vault_ids(
        &self,
        request: StateEntityPageNonFungibleVaultIdsRequest,
    ) -> Result<PageResponse<NonFungibleLocalId>> {
        self.post("state/entity/page/non-fungible-vault/ids", request, res_id)
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
    pub(crate) async fn account_page_resource_preferences(
        &self,
        request: AccountPageResourcePreferencesRequest,
    ) -> Result<PageResponse<AccountResourcePreference>> {
        self.post("state/account/page/resource-preferences", request, res_id)
            .await
    }

    /// Get Account authorized depositors
    ///
    /// Returns paginable collection of authorized depositors for given account.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/AccountAuthorizedDepositorsPage
    pub(crate) async fn account_page_authorized_depositors(
        &self,
        request: AccountPageAuthorizedDepositorsRequest,
    ) -> Result<PageResponse<AccountAuthorizedDepositor>> {
        self.post("state/account/page/authorized-depositors", request, res_id)
            .await
    }
}

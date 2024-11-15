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
        // The GW is currently returning a 404 when this endpoint is called with a virtual account.
        // This is a temporary workaround until the GW is fixed.
        // More info on thread: https://rdxworks.slack.com/archives/C06EBEA0SGY/p1731686360114749
        match self
            .post("state/account/page/resource-preferences", request, res_id)
            .await
        {
            Ok(response) => Ok(response),
            Err(CommonError::NetworkResponseBadCode { code: 404 }) => {
                Ok(PageResponse::empty())
            }
            Err(e) => Err(e),
        }
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
        // The GW is currently returning a 404 when this endpoint is called with a virtual account.
        // This is a temporary workaround until the GW is fixed.
        // More info on thread: https://rdxworks.slack.com/archives/C06EBEA0SGY/p1731686360114749
        match self
            .post("state/account/page/authorized-depositors", request, res_id)
            .await
        {
            Ok(response) => Ok(response),
            Err(CommonError::NetworkResponseBadCode { code: 404 }) => {
                Ok(PageResponse::empty())
            }
            Err(e) => Err(e),
        }
    }
}

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

    /// Get Account resource preferences page
    ///
    /// Returns paginable collection of resource preference rules for given account.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/AccountResourcePreferencesPage
    pub(crate) async fn account_resource_preferences(
        &self,
        request: AccountPageRequest,
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
    pub(crate) async fn account_authorized_depositors(
        &self,
        request: AccountPageRequest,
    ) -> Result<PageResponse<AccountAuthorizedDepositor>> {
        self.post("state/account/page/authorized-depositors", request, res_id)
            .await
    }
}

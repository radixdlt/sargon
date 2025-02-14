use crate::prelude::*;

impl GatewayClient {
    /// Get Gateway status.
    ///
    /// Returns the Gateway current ledger state.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/GatewayStatus
    pub async fn gateway_status(&self) -> Result<GatewayStatusResponse> {
        self.post_empty(Self::PATH_STATUS_GATEWAY_STATUS, res_id)
            .await
    }
}

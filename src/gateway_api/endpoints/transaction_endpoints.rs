use crate::prelude::*;

impl GatewayClient {
    /// Get Construction Metadata
    ///
    /// Returns information needed to construct a new transaction including current epoch number.
    pub(crate) async fn transaction_construction(&self) -> Result<LedgerState> {
        self.post_empty(
            "transaction/construction",
            |response: TransactionConstructionResponse| {
                Ok(response.ledger_state)
            },
        )
        .await
    }

    /// Preview Transaction
    ///
    /// Previews transaction against the network. This endpoint is effectively a
    /// proxy towards the Core API /v0/transaction/preview endpoint.
    ///
    /// See [the Core API documentation][core] for more details.
    ///
    /// [core]: https://radix-babylon-core-api.redoc.ly/#tag/Transaction/paths/~1transaction~1preview/post
    pub(crate) async fn transaction_preview(
        &self,
        request: TransactionPreviewRequest,
    ) -> Result<TransactionPreviewResponse> {
        self.post("transaction/preview", request, res_id).await
    }
}

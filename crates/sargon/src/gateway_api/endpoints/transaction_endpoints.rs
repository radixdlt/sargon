use crate::prelude::*;

impl GatewayClient {
    /// Get Construction Metadata
    ///
    /// Returns information needed to construct a new transaction including current epoch number.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/TransactionConstruction
    pub(crate) async fn transaction_construction(&self) -> Result<LedgerState> {
        self.post_empty(
            "transaction/construction",
            |response: TransactionConstructionResponse| {
                Ok(response.ledger_state)
            },
        )
        .await
    }

    /// Previews transaction against the network - aka "dry run" of transaction.
    ///
    /// Previews are used for [`execution_summary`][TransactionManifest::execution_summary], which the iOS/Android wallet app
    /// uses to present the "review transaction" screen to the user.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/TransactionPreview
    pub(crate) async fn transaction_preview(
        &self,
        request: TransactionPreviewRequest,
    ) -> Result<TransactionPreviewResponse> {
        self.post("transaction/preview", request, res_id).await
    }

    /// Submits a signed transaction payload to the network.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/TransactionSubmit
    pub(crate) async fn transaction_submit(
        &self,
        request: TransactionSubmitRequest,
    ) -> Result<TransactionSubmitResponse> {
        self.post("transaction/submit", request, res_id).await
    }

    /// Observes the status of a transaction.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/TransactionStatus
    pub(crate) async fn transaction_status(
        &self,
        request: TransactionStatusRequest,
    ) -> Result<TransactionStatusResponse> {
        self.post("transaction/status", request, res_id).await
    }

    /// Get Subintent Status
    ///
    /// Returns if the given subintent has been finalized.
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/TransactionSubintentStatus
    pub(crate) async fn subintent_status(
        &self,
        request: SubintentStatusRequest,
    ) -> Result<SubintentStatusResponse> {
        self.post("transaction/subintent-status", request, res_id)
            .await
    }
}

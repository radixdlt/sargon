use crate::prelude::*;

#[uniffi::export]
impl GatewayClient {
    /// Returns the current `Epoch` of the Radix Network of the provided gateway.
    pub async fn current_epoch(&self) -> Result<Epoch> {
        self.transaction_construction()
            .await
            .map(|state| Epoch::from(state.epoch))
    }

    /// Returns the "EncodedReceipt" by running a "dry run" of a
    /// transaction - a preview of the transaction. The "EncodedReceipt" is
    /// required by the [`execution_summary` method](TransactionManifest::execution_summary)
    /// on [`TransactionManifest`].
    pub async fn transaction_dry_run(
        &self,
        intent: TransactionIntent,
        signer_public_keys: Vec<PublicKey>,
    ) -> Result<BagOfBytes> {
        let request =
            TransactionPreviewRequest::new(intent, signer_public_keys, None);
        self.transaction_preview(request)
            .await
            .map(|r| r.encoded_receipt)
            .and_then(|s| BagOfBytes::from_str(&s))
    }
}

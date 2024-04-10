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
    pub async fn dry_run_transaction(
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

    /// Submits a signed transaction payload to the network.
    ///
    /// Returns `Ok(IntentHash)` if the transaction was submitted and not a duplicate.
    pub async fn submit_notarized_transaction(
        &self,
        notarized_transaction: NotarizedTransaction,
    ) -> Result<IntentHash> {
        let intent_hash =
            notarized_transaction.signed_intent().intent().intent_hash();
        let request = TransactionSubmitRequest::new(notarized_transaction);
        self.transaction_submit(request).await.and_then(|r| {
            if r.duplicate {
                Err(CommonError::GatewaySubmitDuplicateTX {
                    intent_hash: intent_hash.to_string(),
                })
            } else {
                Ok(intent_hash)
            }
        })
    }
}

use crate::prelude::*;

impl GatewayClient {
    /// Returns the current `Epoch` of the Radix Network of the provided gateway.
    pub async fn current_epoch(&self) -> Result<Epoch> {
        self.transaction_construction()
            .await
            .map(|state| Epoch::from(state.epoch))
    }

    /// Submits a signed transaction payload to the network.
    ///
    /// Returns `Ok(IntentHash)` if the transaction was submitted and not a duplicate.
    pub async fn submit_notarized_transaction(
        &self,
        notarized_transaction: NotarizedTransaction,
    ) -> Result<TransactionIntentHash> {
        let transaction_intent_hash = notarized_transaction
            .signed_intent()
            .intent()
            .transaction_intent_hash();
        let request = TransactionSubmitRequest::new(notarized_transaction);
        self.submit_transaction(request, transaction_intent_hash)
            .await
    }

    /// Submits a signed transaction payload to the network.
    ///
    /// Returns `Ok(IntentHash)` if the transaction was submitted and not a duplicate.
    pub async fn submit_transaction(
        &self,
        request: TransactionSubmitRequest,
        intent_hash: TransactionIntentHash,
    ) -> Result<TransactionIntentHash> {
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

impl GatewayClient {
    /// Returns the status of a transaction by its `TransactionIntentHash`.
    pub async fn get_transaction_status(
        &self,
        intent_hash: TransactionIntentHash,
    ) -> Result<TransactionStatusResponse> {
        let request = TransactionStatusRequest::new(intent_hash.to_string());
        self.transaction_status(request).await
    }

    /// Returns the `radix_engine_toolkit_receipt` by running a "dry run" of a
    /// transaction - a preview of the transaction. The `radix_engine_toolkit_receipt`` is
    /// required by the [`execution_summary` method](TransactionManifest::execution_summary)
    /// on [`TransactionManifest`].
    pub async fn dry_run_transaction(
        &self,
        intent: TransactionIntent,
        signer_public_keys: Vec<PublicKey>,
    ) -> Result<Option<ScryptoSerializableToolkitTransactionReceipt>> {
        let request = TransactionPreviewRequest::new_transaction_analysis(
            intent.manifest,
            intent.header.start_epoch_inclusive,
            signer_public_keys,
            Some(intent.header.notary_public_key),
            intent.header.nonce,
        );
        self.transaction_preview(request)
            .await
            .map(|r| r.radix_engine_toolkit_receipt)
    }

    /// Returns the status of a pre authorization by its `SubintentHash`.
    pub async fn get_pre_authorization_status(
        &self,
        subintent_hash: SubintentHash,
    ) -> Result<SubintentStatusResponse> {
        let request = SubintentStatusRequest::new(subintent_hash.to_string());
        self.subintent_status(request).await
    }
}

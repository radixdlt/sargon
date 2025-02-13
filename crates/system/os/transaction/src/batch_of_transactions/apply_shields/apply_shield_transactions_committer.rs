use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsCommitter: Send + Sync {
    async fn commit(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: Vec<ManifestWithPayerByAddress>, // TODO: Want IndexSet but not Hash
    ) -> Result<IndexSet<TransactionIntentHash>>;
}

/// Builds, signs and enqueues the transaction intents of manifests
/// which applies a shields to entities.
pub struct ApplyShieldTransactionsCommitterImpl {
    builder: Arc<dyn ApplyShieldTransactionsBuilder>,
    signer: Arc<dyn ApplyShieldTransactionsSigner>,
    enqueuer: Arc<dyn ApplyShieldTransactionsEnqueuer>,
}

impl ApplyShieldTransactionsCommitterImpl {
    pub fn new(os: &SargonOS) -> Result<Self> {
        let builder = ApplyShieldTransactionsBuilderImpl::new(os)?;
        Ok(Self {
            builder: Arc::new(builder),
            signer: Arc::new(ApplyShieldTransactionsSignerImpl::new(os)),
            enqueuer: Arc::new(ApplyShieldTransactionsEnqueuerImpl::new(os)),
        })
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsCommitter for ApplyShieldTransactionsCommitterImpl {
    /// Builds, signs and enqueues the transaction intents of manifests
    /// which applies a shields to entities.
    async fn commit(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: Vec<ManifestWithPayerByAddress>, // TODO: Want IndexSet but not Hash
    ) -> Result<IndexSet<TransactionIntentHash>> {
        let payload_to_sign = self
            .builder
            .build_payload_to_sign(network_id, manifest_and_payer_tuples)
            .await?;

        // Try to sign all applications - we will use  "the best" of
        // the 5 manifests for securified entities
        // This step **also notarizes** the signed intents.
        let signed_transactions = self
            .signer
            .sign_transaction_intents(payload_to_sign)
            .await?;

        // Enqueue all signed transactions
        let transaction_ids = self
            .enqueuer
            .enqueue_signed_transactions(signed_transactions)
            .await?;

        // Return the TransactionIntentHashes
        Ok(transaction_ids)
    }
}

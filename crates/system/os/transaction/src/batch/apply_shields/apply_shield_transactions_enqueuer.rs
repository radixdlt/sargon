use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsEnqueuer: Send + Sync {
    async fn enqueue_signed_transactions(
        &self,
        signed_payload: ApplySecurityShieldSignedPayload,
    ) -> Result<IndexSet<TransactionIntentHash>>;
}

pub struct ApplyShieldTransactionsEnqueuerImpl {}

impl ApplyShieldTransactionsEnqueuerImpl {
    pub fn new(_os: &SargonOS) -> Self {
        warn!("ApplyShieldTransactionsEnqueuerImpl is not implemented yet. We might not need `os` here.");
        Self {}
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsEnqueuer for ApplyShieldTransactionsEnqueuerImpl {
    async fn enqueue_signed_transactions(
        &self,
        signed_payload: ApplySecurityShieldSignedPayload,
    ) -> Result<IndexSet<TransactionIntentHash>> {
        warn!("Enqueuing signed transactions is not implemented yet");
        Ok(signed_payload
            .notarized_transactions
            .into_iter()
            .map(|nt| nt.signed_intent().intent().transaction_intent_hash())
            .collect())
    }
}

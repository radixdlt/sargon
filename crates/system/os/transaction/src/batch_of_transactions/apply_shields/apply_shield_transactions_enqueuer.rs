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
    pub fn new<'a>(os: &'a SargonOS) -> Self {
        todo!()
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsEnqueuer for ApplyShieldTransactionsEnqueuerImpl {
    async fn enqueue_signed_transactions(
        &self,
        signed_payload: ApplySecurityShieldSignedPayload,
    ) -> Result<IndexSet<TransactionIntentHash>> {
        todo!()
    }
}

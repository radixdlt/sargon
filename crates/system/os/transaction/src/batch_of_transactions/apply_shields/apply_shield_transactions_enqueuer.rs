use crate::prelude::*;


#[async_trait::async_trait]
pub trait ApplyShieldTransactionsEnqueuer: shaku::Interface {
    async fn enqueue_signed_transactions(
        &self,
        signed_payload: ApplySecurityShieldSignedPayload,
    ) -> Result<IndexSet<TransactionIntentHash>>;
}

#[derive(Provider)]
#[shaku(interface = ApplyShieldTransactionsEnqueuer)]
pub struct ApplyShieldTransactionsEnqueuerImpl {}

#[async_trait::async_trait]
impl ApplyShieldTransactionsEnqueuer for ApplyShieldTransactionsEnqueuerImpl {
    async fn enqueue_signed_transactions(
        &self,
        signed_payload: ApplySecurityShieldSignedPayload,
    ) -> Result<IndexSet<TransactionIntentHash>> {
        todo!()
    }
}
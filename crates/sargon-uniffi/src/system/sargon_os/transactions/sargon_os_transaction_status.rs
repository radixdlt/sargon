use crate::prelude::*;

// ==================
// Poll Transaction Status (Public)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Polls the state of a Transaction until we can determine its `TransactionStatus`.
    pub async fn poll_transaction_status(
        &self,
        intent_hash: TransactionIntentHash,
    ) -> Result<TransactionStatus> {
        self.wrapped
            .poll_transaction_status(intent_hash.into_internal())
            .await
            .into_result()
    }
}

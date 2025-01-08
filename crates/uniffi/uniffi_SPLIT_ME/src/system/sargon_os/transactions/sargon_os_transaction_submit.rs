use sargon::OsTxSubmitting;

use crate::prelude::*;

// ==================
// Submit Transaction
// ==================
#[uniffi::export]
impl SargonOS {
    /// Submits a notarized transaction payload to the network.
    pub async fn submit_transaction(
        &self,
        notarized_transaction: NotarizedTransaction,
    ) -> Result<TransactionIntentHash> {
        self.wrapped
            .submit_transaction(notarized_transaction.into_internal())
            .await
            .into_result()
    }
}

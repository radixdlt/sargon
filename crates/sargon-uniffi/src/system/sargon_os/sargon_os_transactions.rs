use crate::prelude::*;
use std::time::Duration;

// ==================
// Submit Transaction
// ==================
#[uniffi::export]
impl SargonOS {
    /// Submits a notarized transaction payload to the network.
    pub async fn submit_transaction(
        &self,
        notarized_transaction: NotarizedTransaction,
    ) -> Result<IntentHash> {
        self.wrapped.submit_transaction(notarized_transaction.into_internal()).into_result()
    }
}

// ==================
// Poll Transaction Status (Public)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Polls the state of a Transaction until we can determine its `TransactionStatus`.
    pub async fn poll_transaction_status(
        &self,
        intent_hash: IntentHash,
    ) -> Result<TransactionStatus> {
        self.wrapped.poll_transaction_status(intent_hash.into_internal()).into_result()
    }
}
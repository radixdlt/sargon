use crate::prelude::*;

impl TransactionSubmitRequest {
    pub fn new(notarized_transaction: NotarizedTransaction) -> Self {
        let compiled = notarized_transaction.compile();
        Self {
            notarized_transaction_hex: compiled.to_string(),
        }
    }
}

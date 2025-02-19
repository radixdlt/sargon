use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
/// A Transaction item that is part of the interaction queue.
pub struct TransactionQueueItem {
    /// The identifier of the transaction.
    pub transaction_id: TransactionIntentHash,

    /// Hex-encoded notarized transaction payload which can be submitted to network.
    pub notarized_transaction_hex: String,
}

impl TransactionQueueItem {
    pub fn new(
        transaction_id: TransactionIntentHash,
        notarized_transaction_hex: String,
    ) -> Self {
        Self {
            transaction_id,
            notarized_transaction_hex,
        }
    }
}

impl From<NotarizedTransaction> for TransactionQueueItem {
    fn from(value: NotarizedTransaction) -> Self {
        Self::new(
            value.signed_intent().intent().transaction_intent_hash(),
            value.compile().to_string(),
        )
    }
}

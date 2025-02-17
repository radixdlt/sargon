use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// A Transaction item that is part of the interaction queue.
pub struct TransactionQueueItem {
    /// The identifier of the transaction.
    pub transaction_id: TransactionIntentHash,
}

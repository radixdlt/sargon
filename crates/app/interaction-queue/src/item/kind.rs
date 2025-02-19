use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
/// An enum describing the kind of an item in the interaction queue.
pub enum InteractionQueueItemKind {
    /// A Transaction item.
    Transaction(TransactionQueueItem),

    /// A Pre-Authorization item.
    PreAuthorization(PreAuthorizationQueueItem),
}

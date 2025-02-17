use crate::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
/// An enum describing the status of an item in the interaction queue.
pub enum InteractionQueueItemStatus {
    /// The interaction is queued within a batch and waiting to be processed.
    Queued,

    /// The interaction is the next in line within a batch to be processed.
    Next,

    /// The interaction is currently being processed.
    InProgress,

    /// The interaction was successfully processed.
    Success,

    /// The interaction failed to be processed.
    Failure(InteractionQueueItemFailureStatus),
}

impl PartialOrd for InteractionQueueItemStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for InteractionQueueItemStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_order().cmp(&other.sort_order())
    }
}
impl InteractionQueueItemStatus {
    fn sort_order(&self) -> i32 {
        match self {
            InteractionQueueItemStatus::Failure(_) => 0,
            InteractionQueueItemStatus::Success => 1,
            InteractionQueueItemStatus::InProgress => 2,
            InteractionQueueItemStatus::Next => 3,
            InteractionQueueItemStatus::Queued => 4,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
/// An enum describing the failure status of an item in the interaction queue.
pub enum InteractionQueueItemFailureStatus {
    /// For Transactions only. Can be retried.
    Rejected,

    /// For Transactions only. Cannot be retried.
    Failed,

    /// For Transactions & PreAuthorizations. Cannot be retried.
    TimedOut,
}

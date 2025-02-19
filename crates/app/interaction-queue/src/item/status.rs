use crate::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
/// An enum describing the status of an item in the interaction queue.
pub enum InteractionQueueItemStatus {
    /// The interaction is queued within a batch and waiting to be processed.
    Queued,

    /// The interaction is the next in line within a batch to be processed.
    /// The associated `Instant` is the time when the interaction should be processed.
    Next(Timestamp),

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
            InteractionQueueItemStatus::Next(_) => 3,
            InteractionQueueItemStatus::Queued => 4,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
/// An enum describing the failure status of an item in the interaction queue.
pub enum InteractionQueueItemFailureStatus {
    /// For Transactions only. Can be retried.
    Rejected,

    /// For Transactions only. Cannot be retried.
    Failed,

    /// For Transactions & PreAuthorizations. Cannot be retried.
    TimedOut,
}

impl From<TransactionStatusResponsePayloadStatus>
    for InteractionQueueItemStatus
{
    fn from(value: TransactionStatusResponsePayloadStatus) -> Self {
        match value {
            TransactionStatusResponsePayloadStatus::Unknown |
            TransactionStatusResponsePayloadStatus::Pending |
            TransactionStatusResponsePayloadStatus::CommitPendingOutcomeUnknown => {
                Self::InProgress
            }
            TransactionStatusResponsePayloadStatus::CommittedSuccess => {
                Self::Success
            }
            TransactionStatusResponsePayloadStatus::CommittedFailure => {
                // TODO: Review failure type
                Self::Failure(InteractionQueueItemFailureStatus::TimedOut)
            }
            TransactionStatusResponsePayloadStatus::PermanentlyRejected => {
                // TODO: Review failure type
                Self::Failure(InteractionQueueItemFailureStatus::Rejected)
            }
            TransactionStatusResponsePayloadStatus::TemporarilyRejected => {
                // TODO: Review failure type
                Self::Failure(InteractionQueueItemFailureStatus::Failed)
            }
        }
    }
}

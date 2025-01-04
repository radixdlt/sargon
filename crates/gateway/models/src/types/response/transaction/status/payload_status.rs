use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum TransactionStatusResponsePayloadStatus {
    Unknown,
    CommittedSuccess,
    CommittedFailure,
    CommitPendingOutcomeUnknown,
    PermanentlyRejected,
    TemporarilyRejected,
    Pending,
}

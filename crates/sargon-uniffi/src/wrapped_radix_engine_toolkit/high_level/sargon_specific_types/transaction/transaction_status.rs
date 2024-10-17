use crate::prelude::*;
use sargon::TransactionStatus as InternalTransactionStatus;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
    uniffi::Enum,
)]
pub enum TransactionStatus {
    /// The transaction has been successfully processed and is now final.
    Success,

    /// The transaction has been permanently rejected with the given `reason`.
    PermanentlyRejected { reason: TransactionStatusReason },

    /// The transaction has been temporarily rejected and may be processed in the future.
    TemporarilyRejected { current_epoch: Epoch },

    /// The transaction has failed with the given `reason`.
    Failed { reason: TransactionStatusReason },
}
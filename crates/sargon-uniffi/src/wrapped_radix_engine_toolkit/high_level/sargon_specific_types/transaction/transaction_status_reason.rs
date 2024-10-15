use crate::prelude::*;
use sargon::TransactionStatusReason as InternalTransactionStatusReason;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    InternalConversionV2,
    uniffi::Enum,
)]
pub enum TransactionStatusReason {
    /// The transaction was rejected for an unknown reason.
    Unknown,

    /// The transaction was rejected because there was an application error in the worktop.
    WorktopError,
}
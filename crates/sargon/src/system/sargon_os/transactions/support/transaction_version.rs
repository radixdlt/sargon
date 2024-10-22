use crate::prelude;

/// This enum is used to specify the version of the transaction.
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum TransactionVersion {
    /// Regular transactions
    V1,
    /// Pre-authorized transactions
    V2,
}

use crate::prelude::*;

#[derive(
    Deserialize, Serialize, Clone, PartialEq, Eq, Debug, derive_more::Display,
)]
pub enum TransactionReceiptStatus {
    Succeeded,
    Failed,
    Rejected,
}

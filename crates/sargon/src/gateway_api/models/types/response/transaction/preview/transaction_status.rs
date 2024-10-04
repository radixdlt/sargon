use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::Display,
    uniffi::Enum,
)]
pub enum TransactionStatus {
    Succeeded,
    Failed,
    Rejected,
}

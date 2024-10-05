use crate::prelude::*;

/// User Preferences relating to submission of transactions.
#[derive(
    Debug,
    Copy,
    PartialEq,
    Eq,
    Clone,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("default guarantee: {}", default_deposit_guarantee)]
pub struct TransactionPreferences {
    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    pub default_deposit_guarantee: Decimal192,
}
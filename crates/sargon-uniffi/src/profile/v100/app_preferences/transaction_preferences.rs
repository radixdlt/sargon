use crate::prelude::*;
use sargon::TransactionPreferences as InternalTransactionPreferences;

/// User Preferences relating to submission of transactions.
#[derive(PartialEq, Eq, Clone, Hash, InternalConversionV2, uniffi::Record)]
pub struct TransactionPreferences {
    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    pub default_deposit_guarantee: Decimal192,
}
use crate::prelude::*;
use sargon::TransactionPreferences as InternalTransactionPreferences;

/// User Preferences relating to submission of transactions.
#[derive(
    Debug,
    
    PartialEq,
    Eq,
    Clone,
    Hash,
     uniffi::Record,
)]
pub struct TransactionPreferences {
    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    pub default_deposit_guarantee: Decimal192,
}

impl From<InternalTransactionPreferences> for TransactionPreferences {
    fn from(value: InternalTransactionPreferences) -> Self {
        Self {
            default_deposit_guarantee: value.default_deposit_guarantee.into(),
        }
    }
}

impl Into<InternalTransactionPreferences> for TransactionPreferences {
    fn into(self) -> InternalTransactionPreferences {
        InternalTransactionPreferences {
            default_deposit_guarantee: self.default_deposit_guarantee.into(),
        }
    }
}
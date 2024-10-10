use crate::prelude::*;
use sargon::LedgerHardwareWalletHint as InternalLedgerHardwareWalletHint;

#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct LedgerHardwareWalletHint {
    /// "Orange, scratched"
    pub name: String,

    /// E.g. `nanoS+`
    pub model: LedgerHardwareWalletModel,
}

impl From<InternalLedgerHardwareWalletHint> for LedgerHardwareWalletHint {
    fn from(value: InternalLedgerHardwareWalletHint) -> Self {
        Self {
            name: value.name,
            model: value.model.into(),
        }
    }
}

impl Into<InternalLedgerHardwareWalletHint> for LedgerHardwareWalletHint {
    fn into(self) -> InternalLedgerHardwareWalletHint {
        InternalLedgerHardwareWalletHint {
            name: self.name,
            model: self.model.into(),
        }
    }
}


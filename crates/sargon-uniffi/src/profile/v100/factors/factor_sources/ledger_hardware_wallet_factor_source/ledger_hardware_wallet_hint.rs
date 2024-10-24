use crate::prelude::*;
use sargon::LedgerHardwareWalletHint as InternalLedgerHardwareWalletHint;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct LedgerHardwareWalletHint {
    /// "Orange, scratched"
    pub name: String,

    /// E.g. `nanoS+`
    pub model: LedgerHardwareWalletModel,
}

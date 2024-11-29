use crate::prelude::*;
use sargon::LedgerHardwareWalletHint as InternalLedgerHardwareWalletHint;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct LedgerHardwareWalletHint {
    /// A user-assigned name for the ledger, intended to help users
    /// differentiate between multiple ledgers.
    ///
    /// E.g. "Orange, scratched"
    pub label: String,

    /// E.g. `nanoS+`
    pub model: LedgerHardwareWalletModel,
}

use crate::prelude::*;
use sargon::SpotCheckResponse as InternalSpotCheckResponse;

/// The purpose of the authorization request
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SpotCheckResponse {
    /// The user retrieved the id of a Ledger device.
    /// Used for the identification of `LedgerHardwareWalletFactorSource`.
    Ledger { id: Exactly32Bytes },

    /// The user retrieved a `MnemonicWithPassphrase`.
    /// Used for the identification of any other `FactorSource`.
    MnemonicWithPassphrase { value: MnemonicWithPassphrase },
}

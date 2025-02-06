use crate::prelude::*;
use sargon::SpotCheckResponse as InternalSpotCheckResponse;

/// The purpose of the authorization request
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SpotCheckResponse {
    /// The user retrieved the id of a Ledger device.
    /// Used for the identification of `LedgerHardwareWalletFactorSource`.
    Ledger { id: Exactly32Bytes },

    /// The user retrieved the `FactorSourceIdFromHash` that identified an Arculus card.
    /// /// Used for the identification of `ArculusCardFactorSource`.
    ArculusCard { id: FactorSourceIDFromHash },

    /// The user retrieved a `MnemonicWithPassphrase`.
    /// Used for the identification of any software `FactorSource`.
    Software {
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    },
}

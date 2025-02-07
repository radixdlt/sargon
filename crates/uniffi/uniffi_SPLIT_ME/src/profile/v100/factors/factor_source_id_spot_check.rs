use crate::prelude::*;
use factors::FactorSourceIDSpotCheck;
use sargon::FactorSourceID as InternalFactorSourceID;
use sargon::SpotCheckInput as InternalSpotCheckInput;

/// An enum with the input to perform a spot check for a given `FactorSourceID`.
/// This is, to validate that the `FactorSourceID` was created with the same input that has been provided.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SpotCheckInput {
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

#[uniffi::export]
pub fn factor_source_id_perform_spot_check(
    factor_source_id: FactorSourceID,
    input: SpotCheckInput,
) -> bool {
    factor_source_id
        .into_internal()
        .perform_spot_check(input.into_internal())
}

#[uniffi::export]
pub fn factor_source_perform_spot_check(
    factor_source: FactorSource,
    input: SpotCheckInput,
) -> bool {
    factor_source
        .into_internal()
        .perform_spot_check(input.into_internal())
}

#[uniffi::export]
pub fn factor_source_id_from_hash_perform_spot_check(
    factor_source_id_from_hash: FactorSourceIDFromHash,
    input: SpotCheckInput,
) -> bool {
    factor_source_id_from_hash
        .into_internal()
        .perform_spot_check(input.into_internal())
}

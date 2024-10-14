use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::LedgerHardwareWalletFactorSource as InternalLedgerHardwareWalletFactorSource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct LedgerHardwareWalletFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic,
    /// that is secured by the Ledger Hardware Wallet device.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a LedgerHardwareWalletFactorSource to help user disambiguate between it and another one.
    pub hint: LedgerHardwareWalletHint,
}

#[uniffi::export]
pub fn new_ledger_hardware_wallet_factor_source_sample(
) -> LedgerHardwareWalletFactorSource {
    InternalLedgerHardwareWalletFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_ledger_hardware_wallet_factor_source_sample_other(
) -> LedgerHardwareWalletFactorSource {
    InternalLedgerHardwareWalletFactorSource::sample_other().into()
}

#[uniffi::export]
fn new_ledger_hardware_wallet_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase,
    hint: LedgerHardwareWalletHint,
    common: FactorSourceCommon,
) -> LedgerHardwareWalletFactorSource {
    let id = InternalFactorSourceIDFromHash::new_for_ledger(&mwp.into());
    InternalLedgerHardwareWalletFactorSource::new(
        id,
        common.into(),
        hint.into(),
    )
    .into()
}

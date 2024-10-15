use crate::prelude::*;
use sargon::PassphraseFactorSource as InternalPassphraseFactorSource;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;

/// NOT IMPLEMENTED NOR USED YET
///
/// A passphrase based FactorSource is essentially a Input Key Material based Mnemonic,
/// user needs to input the passphrase - key material - every time they use this factor source
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    InternalConversionV2,
    uniffi::Record,
)]
pub struct PassphraseFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,
}

#[uniffi::export]
pub fn new_passphrase_factor_source_sample() -> PassphraseFactorSource {
    InternalPassphraseFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_passphrase_factor_source_sample_other() -> PassphraseFactorSource {
    InternalPassphraseFactorSource::sample_other().into()
}

#[uniffi::export]
fn new_passphrase_factor_source_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase,
) -> PassphraseFactorSource {
    let id = InternalFactorSourceIDFromHash::new_for_passphrase(&mwp.into_internal());
    InternalPassphraseFactorSource::new(id).into()
}
use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::PasswordFactorSource as InternalPasswordFactorSource;

/// NOT IMPLEMENTED NOR USED YET
///
/// A passphrase based FactorSource is essentially a Input Key Material based Mnemonic,
/// user needs to input the passphrase - key material - every time they use this factor source
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct PasswordFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a PasswordFactorSource to help user
    /// disambiguate between it and another one.
    pub hint: PasswordFactorSourceHint,
}

#[uniffi::export]
pub fn new_password_factor_source_sample() -> PasswordFactorSource {
    InternalPasswordFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_password_factor_source_sample_other() -> PasswordFactorSource {
    InternalPasswordFactorSource::sample_other().into()
}

#[uniffi::export]
fn new_password_factor_source_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase,
    hint: PasswordFactorSourceHint,
) -> PasswordFactorSource {
    let id =
        InternalFactorSourceIDFromHash::new_for_password(&mwp.into_internal());
    InternalPasswordFactorSource::new(id, hint.into()).into()
}

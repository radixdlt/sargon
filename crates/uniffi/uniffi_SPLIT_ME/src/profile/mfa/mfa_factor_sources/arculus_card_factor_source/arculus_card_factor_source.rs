use crate::prelude::*;
use sargon::ArculusCardFactorSource as InternalArculusCardFactorSource;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;

/// An Arculus card, a hierarchal deterministic wallet capable of CAP26 derivation
/// which users interact with by placing it near their host device, which
/// communicates with the card over NFC.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ArculusCardFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic,
    /// that is secured by the Arculus Card.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a ArculusCardFactorSource to help user disambiguate
    /// between it and another one.
    pub hint: ArculusCardHint,
}

delegate_debug_into!(ArculusCardFactorSource, InternalArculusCardFactorSource);

#[uniffi::export]
pub fn new_arculus_card_factor_source_sample() -> ArculusCardFactorSource {
    InternalArculusCardFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_arculus_card_factor_source_sample_other() -> ArculusCardFactorSource
{
    InternalArculusCardFactorSource::sample_other().into()
}

#[uniffi::export]
fn new_arculus_card_factor_source_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase,
    hint: ArculusCardHint,
) -> ArculusCardFactorSource {
    let id =
        InternalFactorSourceIDFromHash::new_for_arculus_with_mwp(&mwp.into());
    InternalArculusCardFactorSource::new(id, hint.into()).into()
}

use crate::prelude::*;
use crate::prelude::*;

/// An Arculus card, a hierarchal deterministic wallet capable of CAP26 derivation
/// which users interact with by placing it near their host device, which
/// communicates with the card over NFC.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{hint} : {id}")]
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

#[uniffi::export]
pub fn new_arculus_card_factor_source_sample() -> ArculusCardFactorSource {
    ArculusCardFactorSource::sample()
}

#[uniffi::export]
pub fn new_arculus_card_factor_source_sample_other() -> ArculusCardFactorSource
{
    ArculusCardFactorSource::sample_other()
}

#[uniffi::export]
fn new_arculus_card_factor_source_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase,
    hint: ArculusCardHint,
) -> ArculusCardFactorSource {
    let id = FactorSourceIDFromHash::new_for_arculus(&mwp);
    ArculusCardFactorSource::new(id, hint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ArculusCardFactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_arculus_card_factor_source_sample(),
                new_arculus_card_factor_source_sample_other(),
                // duplicates should get removed
                new_arculus_card_factor_source_sample(),
                new_arculus_card_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_arculus_card_factor_source_from_mnemonic_with_passphrase() {
        assert_eq!(
            new_arculus_card_factor_source_from_mnemonic_with_passphrase(
                MnemonicWithPassphrase::sample_arculus(),
                ArculusCardHint::sample()
            )
            .factor_source_id(),
            SUT::sample().factor_source_id()
        );
    }
}

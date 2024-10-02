use crate::prelude::*;

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

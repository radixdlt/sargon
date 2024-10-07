use crate::prelude::*;

#[uniffi::export]
pub fn new_passphrase_factor_source_sample() -> PassphraseFactorSource {
    PassphraseFactorSource::sample()
}

#[uniffi::export]
pub fn new_passphrase_factor_source_sample_other() -> PassphraseFactorSource
{
    PassphraseFactorSource::sample_other()
}

#[uniffi::export]
fn new_passphrase_factor_source_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase
) -> PassphraseFactorSource {
    let id = FactorSourceIDFromHash::new_for_passphrase(&mwp);
    PassphraseFactorSource::new(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PassphraseFactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_passphrase_factor_source_sample(),
                new_passphrase_factor_source_sample_other(),
                // duplicates should get removed
                new_passphrase_factor_source_sample(),
                new_passphrase_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_arculus_card_factor_source_from_mnemonic_with_passphrase() {
        assert_eq!(
            new_passphrase_factor_source_from_mnemonic_with_passphrase(
                MnemonicWithPassphrase::sample_passphrase(),
            )
            .factor_source_id(),
            SUT::sample().factor_source_id()
        );
    }
}

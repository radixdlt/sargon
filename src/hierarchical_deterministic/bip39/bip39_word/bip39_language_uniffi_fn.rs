use crate::prelude::*;

#[uniffi::export]
pub fn new_bip39_language_sample() -> BIP39Language {
    BIP39Language::sample()
}

#[uniffi::export]
pub fn new_bip39_language_sample_other() -> BIP39Language {
    BIP39Language::sample_other()
}

#[uniffi::export]
pub fn bip39_language_wordlist(language: &BIP39Language) -> Vec<BIP39Word> {
    language.wordlist()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39Language;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_bip39_language_sample(),
                new_bip39_language_sample_other(),
                // duplicates should get removed
                new_bip39_language_sample(),
                new_bip39_language_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_word_list() {
        assert_eq!(
            bip39_language_wordlist(&SUT::sample()),
            SUT::sample().wordlist()
        )
    }
}

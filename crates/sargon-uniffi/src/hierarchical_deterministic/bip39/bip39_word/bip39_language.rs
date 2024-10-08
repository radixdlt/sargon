use crate::prelude::*;
use sargon::BIP39Language as InternalBIP39Language;

/// Language to be used for the mnemonic phrase.
///
/// The English language is always available, other languages are enabled using
/// the compilation features.
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    uniffi::Enum,
)]
pub enum BIP39Language {
    /// The English language.
    English,

    /// The French language.
    French,
}

impl From<InternalBIP39Language> for BIP39Language {
    fn from(value: InternalBIP39Language) -> Self {
        match value {
            InternalBIP39Language::English => Self::English,
            InternalBIP39Language::French => Self::French,
        }
    }
}

impl Into<InternalBIP39Language> for BIP39Language {
    fn into(self) -> InternalBIP39Language {
        match self {
            Self::English => InternalBIP39Language::English,
            Self::French => InternalBIP39Language::French,
        }
    }
}

#[uniffi::export]
pub fn new_bip39_language_sample() -> BIP39Language {
    InternalBIP39Language::sample().into()
}

#[uniffi::export]
pub fn new_bip39_language_sample_other() -> BIP39Language {
    InternalBIP39Language::sample_other().into()
}

#[uniffi::export]
pub fn bip39_language_wordlist(language: &BIP39Language) -> Vec<BIP39Word> {
    language.into::<InternalBIP39Language>().wordlist().into_iter().map(Into::into).collect()
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

use crate::prelude::*;

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
    PartialOrd,
    Ord,
    derive_more::Display,
)]
pub enum BIP39Language {
    /// The English language.
    English,

    /// The French language.
    French,
}

impl Default for BIP39Language {
    fn default() -> Self {
        Self::English
    }
}

impl HasSampleValues for BIP39Language {
    fn sample() -> Self {
        Self::English
    }

    fn sample_other() -> Self {
        Self::French
    }
}

impl BIP39Language {
    pub fn wordlist(&self) -> Vec<BIP39Word> {
        let language: bip39::Language = (*self).into();
        let word_list = language.word_list();

        word_list
            .iter()
            .map(|w| BIP39Word::new(w, *self))
            .collect::<Result<Vec<BIP39Word>, CommonError>>()
            .expect("Crate bip39 generated words unknown to us.")
    }
}

impl From<bip39::Language> for BIP39Language {
    fn from(value: bip39::Language) -> Self {
        use bip39::Language::*;
        match value {
            English => Self::English,
            French => Self::French,
        }
    }
}
impl From<BIP39Language> for bip39::Language {
    fn from(value: BIP39Language) -> Self {
        use bip39::Language::*;
        match value {
            BIP39Language::English => English,
            BIP39Language::French => French,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39Language;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn default_is_english() {
        assert_eq!(SUT::default(), SUT::English);
    }

    #[test]
    fn test_wordlist_english() {
        let wordlist = SUT::English.wordlist();
        assert_eq!(wordlist.into_iter().last().unwrap().word, "zoo");
    }

    #[test]
    fn test_wordlist_french() {
        let wordlist = SUT::French.wordlist();
        assert_eq!(wordlist.into_iter().last().unwrap().word, "zoologie");
    }

    #[test]
    fn to_from_bip39_english() {
        assert_eq!(SUT::English, bip39::Language::English.into());
        assert_eq!(
            bip39::Language::from(SUT::English),
            bip39::Language::English
        );
    }

    #[test]
    fn to_from_bip39_french() {
        assert_eq!(SUT::French, bip39::Language::French.into());
        assert_eq!(bip39::Language::from(SUT::French), bip39::Language::French);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::English), "English");
    }
}

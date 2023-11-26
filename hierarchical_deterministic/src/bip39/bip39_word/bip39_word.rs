use std::cmp::Ordering;

use bip39::Language;
use memoize::memoize;
use wallet_kit_common::error::Error;

use super::u11::U11;

/// A word in the BIP39 word list of `language` at known `index` (0-2047).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BIP39Word {
    pub word: String,
    pub index: U11,
    pub language: Language,
}

impl Ord for BIP39Word {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for BIP39Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BIP39Word {
    pub fn new(word: &'static str, language: Language) -> Result<Self, Error> {
        let index = index_of_word_in_bip39_wordlist_of_language(&word, language)
            .ok_or(Error::UnknownBIP39Word)?;
        Ok(Self {
            word: word.to_string(),
            index,
            language,
        })
    }
    pub fn english(word: &'static str) -> Result<Self, Error> {
        Self::new(word, Language::English)
    }
}

#[memoize]
fn index_of_word_in_bip39_wordlist_of_language(
    word: &'static str,
    language: Language,
) -> Option<U11> {
    language
        .find_word(word)
        .map(|i| U11::new(i).expect("Less than 2048"))
}

#[cfg(test)]
mod tests {
    use bip39::Language;
    use wallet_kit_common::error::Error;

    use super::BIP39Word;

    #[test]
    fn equality() {
        assert_eq!(BIP39Word::english("zoo"), BIP39Word::english("zoo"));
    }

    #[test]
    fn language_of_zoo_is_english() {
        assert_eq!(
            BIP39Word::english("zoo").unwrap().language,
            Language::English
        );
    }

    #[test]
    fn invalid_word() {
        assert_eq!(BIP39Word::english("foobar"), Err(Error::UnknownBIP39Word));
    }

    #[test]
    fn index_of_zoo_is_2047() {
        assert_eq!(BIP39Word::english("zoo").unwrap().index.into_inner(), 2047);
    }

    #[test]
    fn ord() {
        assert!(BIP39Word::english("abandon").unwrap() < BIP39Word::english("ability").unwrap());
        assert!(BIP39Word::english("zoo").unwrap() > BIP39Word::english("zone").unwrap());
    }
}

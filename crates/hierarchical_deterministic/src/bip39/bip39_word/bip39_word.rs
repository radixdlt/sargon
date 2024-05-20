use crate::prelude::*;

/// A word in the BIP39 word list of `language` at known `index` (0-2047).
#[derive(Zeroize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct BIP39Word {
    pub word: String,
    pub index: U11,

    #[zeroize(skip)]
    pub language: BIP39Language,
}

impl HasSampleValues for BIP39Word {
    fn sample() -> Self {
        Self::english("abandon").unwrap()
    }

    fn sample_other() -> Self {
        Self::english("zoo").unwrap()
    }
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
    pub fn new(word: &'static str, language: BIP39Language) -> Result<Self> {
        let index =
            index_of_word_in_bip39_wordlist_of_language(word, language.into())
                .ok_or(CommonError::UnknownBIP39Word)?;
        Ok(Self {
            word: word.to_string(),
            index,
            language,
        })
    }
    pub fn english(word: &'static str) -> Result<Self> {
        Self::new(word, BIP39Language::English)
    }
}

fn index_of_word_in_bip39_wordlist_of_language(
    word: &'static str,
    language: bip39::Language,
) -> Option<U11> {
    language
        .find_word(word)
        .map(|i| U11::new(i).expect("Less than 2048"))
}

fn word_by_index(u11: U11, language: bip39::Language) -> BIP39Word {
    let index = u11.inner as usize;
    let word = language.word_list()[index];
    BIP39Word {
        word: word.to_owned(),
        index: u11,
        language: language.into(),
    }
}

pub(crate) fn bip39_word_by_index(u11: U11) -> BIP39Word {
    word_by_index(u11, bip39::Language::English)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39Word;

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
    fn language_of_zoo_is_english() {
        assert_eq!(
            SUT::english("zoo").unwrap().language,
            BIP39Language::English
        );
    }

    #[test]
    fn invalid_word() {
        assert_eq!(SUT::english("foobar"), Err(CommonError::UnknownBIP39Word));
    }

    #[test]
    fn index_of_zoo_is_2047() {
        assert_eq!(SUT::english("zoo").unwrap().index.inner, 2047);
    }

    #[test]
    fn ord() {
        assert!(
            SUT::english("abandon").unwrap() < SUT::english("ability").unwrap()
        );
        assert!(SUT::english("zoo").unwrap() > SUT::english("zone").unwrap());
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::english("zoo").unwrap();
        let copy = sut.clone();
        sut.zeroize();
        assert_ne!(sut, copy);
    }
}

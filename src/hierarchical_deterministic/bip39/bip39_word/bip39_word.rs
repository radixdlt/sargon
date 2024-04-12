use crate::prelude::*;

/// A word in the BIP39 word list of `language` at known `index` (0-2047).
#[derive(Zeroize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct BIP39Word {
    pub word: String,
    pub index: U11,
    
    #[zeroize(skip)]
    pub language: BIP39Language,
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
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            BIP39Word::english("zoo").unwrap(),
            BIP39Word::english("zoo").unwrap()
        );
    }

    #[test]
    fn word() {
        assert_eq!(BIP39Word::english("zoo").unwrap().word, "zoo");
    }

    #[test]
    fn language_of_zoo_is_english() {
        assert_eq!(
            BIP39Word::english("zoo").unwrap().language,
            BIP39Language::English
        );
    }

    #[test]
    fn invalid_word() {
        assert_eq!(
            BIP39Word::english("foobar"),
            Err(CommonError::UnknownBIP39Word)
        );
    }

    #[test]
    fn index_of_zoo_is_2047() {
        assert_eq!(BIP39Word::english("zoo").unwrap().index.inner, 2047);
    }

    #[test]
    fn ord() {
        assert!(
            BIP39Word::english("abandon").unwrap()
                < BIP39Word::english("ability").unwrap()
        );
        assert!(
            BIP39Word::english("zoo").unwrap()
                > BIP39Word::english("zone").unwrap()
        );
    }
}

use crate::prelude::*;

pub(crate) struct CommonMnemonicBuilder;

impl CommonMnemonicBuilder {
    /// Creates a new mnemonic from given `words`
    /// Returns `InvalidMnemonicWords` error if any of the words are invalid
    pub fn create_mnemonic_from_words(words: Vec<String>) -> Result<Mnemonic> {
        let (bip39_words, invalid_words): (Vec<_>, Vec<_>) = words
            .iter()
            .enumerate()
            .map(|(index, w)| (index, BIP39Word::english(w)))
            .partition(|(_, word)| word.is_ok());

        let mnemonic_indices_of_invalid_words = invalid_words
            .into_iter()
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        if !mnemonic_indices_of_invalid_words.is_empty() {
            return Err(CommonError::InvalidMnemonicWords {
                indices_in_mnemonic: mnemonic_indices_of_invalid_words,
            });
        }

        let bip39_words = bip39_words
            .into_iter()
            .map(|(_, word)| word.unwrap())
            .collect::<Vec<BIP39Word>>();
        Mnemonic::from_words(bip39_words)
    }
}

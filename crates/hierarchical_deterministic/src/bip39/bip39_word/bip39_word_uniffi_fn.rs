use crate::prelude::*;

#[uniffi::export]
pub fn new_bip39_word_sample() -> BIP39Word {
    BIP39Word::sample()
}

#[uniffi::export]
pub fn new_bip39_word_sample_other() -> BIP39Word {
    BIP39Word::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39Word;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_bip39_word_sample(),
                new_bip39_word_sample_other(),
                // duplicates should get removed
                new_bip39_word_sample(),
                new_bip39_word_sample_other(),
            ])
            .len(),
            2
        );
    }
}

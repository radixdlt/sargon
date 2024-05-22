use crate::prelude::*;

// #[uniffi::export]
// pub fn new_mnemonic_generate_with_entropy(
//     entropy: BIP39Entropy,
//     language: BIP39Language,
// ) -> Mnemonic {
//     Mnemonic::from_entropy_in(entropy, language)
// }

/// Returns new mnemonic from a string of words
#[uniffi::export]
pub fn new_mnemonic_from_phrase(phrase: String) -> Result<Mnemonic> {
    Mnemonic::from_phrase(&phrase)
}

#[uniffi::export]
pub fn new_mnemonic_from_phrase_language(
    phrase: String,
    language: BIP39Language,
) -> Result<Mnemonic> {
    Mnemonic::from(&phrase, language)
}

#[uniffi::export]
pub fn new_mnemonic_from_words(words: Vec<BIP39Word>) -> Result<Mnemonic> {
    Mnemonic::from_words(words)
}

/// Returns the words of a mnemonic as a String joined by spaces, e.g. "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
#[uniffi::export]
pub fn mnemonic_phrase(from: &Mnemonic) -> String {
    from.phrase()
}

#[uniffi::export]
pub fn new_mnemonic_sample() -> Mnemonic {
    Mnemonic::sample()
}

#[uniffi::export]
pub fn new_mnemonic_sample_other() -> Mnemonic {
    Mnemonic::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Mnemonic;

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_mnemonic_sample(),
                new_mnemonic_sample_other(),
                // duplicates should be removed
                new_mnemonic_sample(),
                new_mnemonic_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_mnemonic_phrase() {
        let str = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong";
        let sut: SUT = str.parse().unwrap();
        assert_eq!(mnemonic_phrase(&sut), str);
    }

    // #[test]
    // fn test_new_mnemonic_generate_with_entropy_16_bytes() {
    //     let sut = new_mnemonic_generate_with_entropy(
    //         BIP39Entropy::EntropyOf16Bytes(Entropy16Bytes::new([0xff; 16])),
    //         BIP39Language::English,
    //     );
    //     assert_eq!(
    //         sut.phrase(),
    //         "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
    //     );
    // }

    // #[test]
    // fn test_new_mnemonic_generate_with_entropy_32_bytes() {
    //     let sut = new_mnemonic_generate_with_entropy(
    //         BIP39Entropy::EntropyOf32Bytes(Entropy32Bytes::new([0xff; 32])),
    //         BIP39Language::English,
    //     );
    //     assert_eq!(sut.phrase(), "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
    // );
    // }

    #[test]
    fn test_new_mnemonic_from_phrase() {
        let str =
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong".to_string();
        let mnemonic = new_mnemonic_from_phrase(str.clone()).unwrap();
        assert_eq!(mnemonic_phrase(&mnemonic), str)
    }

    #[test]
    fn test_new_mnemonic_from_phrase_language() {
        let str =
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong".to_string();
        let mnemonic = new_mnemonic_from_phrase_language(
            str.clone(),
            BIP39Language::English,
        )
        .unwrap();
        assert_eq!(mnemonic_phrase(&mnemonic), str)
    }

    #[test]
    fn test_new_mnemonic_from_words() {
        let str = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong";
        let words = str
            .split(' ')
            .map(|w| BIP39Word::new(w, BIP39Language::English).unwrap())
            .collect_vec();
        let mnemonic = new_mnemonic_from_words(words).unwrap();
        assert_eq!(mnemonic_phrase(&mnemonic), str)
    }
}

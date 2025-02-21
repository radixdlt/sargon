use crate::prelude::*;

/// A builder of `MnemonicWithPassphrase` required for a new `DeviceFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug)]
pub struct DeviceMnemonicBuilder {
    mnemonic_with_passphrase: RwLock<Option<MnemonicWithPassphrase>>,
}

/// The outcome of the `build` function from `DeviceMnemonicBuilder`.
#[derive(Debug, PartialEq)]
pub enum DeviceMnemonicBuildOutcome {
    /// The mnemonic words were confirmed
    Confirmed {
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    },
    /// The mnemonic words were unconfirmed
    Unconfirmed {
        indices_in_mnemonic: IndexSet<usize>,
    },
}

impl Default for DeviceMnemonicBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for DeviceMnemonicBuilder {
    fn eq(&self, other: &Self) -> bool {
        let mnemonic_with_passphrase =
            self.mnemonic_with_passphrase.read().unwrap();
        let other_mnemonic_with_passphrase =
            other.mnemonic_with_passphrase.read().unwrap();
        *mnemonic_with_passphrase == *other_mnemonic_with_passphrase
    }
}

impl Eq for DeviceMnemonicBuilder {}

impl std::hash::Hash for DeviceMnemonicBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mnemonic_with_passphrase =
            self.mnemonic_with_passphrase.read().unwrap();
        mnemonic_with_passphrase.hash(state);
    }
}

impl HasSampleValues for DeviceMnemonicBuilder {
    fn sample() -> Self {
        Self::with_mnemonic_with_passphrase(MnemonicWithPassphrase::sample())
    }

    fn sample_other() -> Self {
        Self::with_mnemonic_with_passphrase(
            MnemonicWithPassphrase::sample_other(),
        )
    }
}

impl DeviceMnemonicBuilder {
    fn new() -> Self {
        Self {
            mnemonic_with_passphrase: RwLock::new(None),
        }
    }

    pub fn with_mnemonic_with_passphrase(
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    ) -> Self {
        Self {
            mnemonic_with_passphrase: RwLock::new(Some(
                mnemonic_with_passphrase,
            )),
        }
    }
}

// ====================
// ==== GET / READ ====
// ====================
impl DeviceMnemonicBuilder {
    /// Returns the words of the mnemonic with passphrase.
    pub fn get_words(&self) -> Vec<BIP39Word> {
        self.get_mnemonic_with_passphrase().mnemonic.words
    }

    /// Returns the `mnemonic_with_passphrase` if it was previously created or panics.
    fn get_mnemonic_with_passphrase(&self) -> MnemonicWithPassphrase {
        self.mnemonic_with_passphrase
            .read()
            .unwrap()
            .clone()
            .expect("Mnemonic with passphrase should be created first")
    }
}

// ====================
// ===== MUTATION =====
// ====================
impl DeviceMnemonicBuilder {
    /// Generates a new mnemonic
    pub fn generate_new_mnemonic(&self) -> &Self {
        let mnemonic = Mnemonic::generate_new();
        self.set_mnemonic_with_passphrase(mnemonic);
        self
    }

    /// Creates a new mnemonic from given `words`
    pub fn create_mnemonic_from_words(
        &self,
        words: Vec<String>,
    ) -> Result<&Self> {
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
        let mnemonic = Mnemonic::from_words(bip39_words)?;
        self.set_mnemonic_with_passphrase(mnemonic);
        Ok(self)
    }

    fn set_mnemonic_with_passphrase(&self, mnemonic: Mnemonic) {
        *self.mnemonic_with_passphrase.write().unwrap() =
            Some(MnemonicWithPassphrase::new(mnemonic));
    }
}

impl DeviceMnemonicBuilder {
    const NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM:
        usize = 3;
    const NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM: usize =
        Self::NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM
            + 1;

    /// Get a set of words indices within `MnemonicWithPassphrase` to be confirmed.
    /// Always includes the last mnemonic word index.
    pub fn get_indices_in_mnemonic_of_words_to_confirm(
        &self,
    ) -> IndexSet<usize> {
        let max_index_in_mnemonic =
            self.get_mnemonic_with_passphrase().mnemonic.words.len() - 1;

        let mut indices_in_mnemonic_of_words_to_confirm = generate_bytes::<100>()
            .into_iter()
            .map(|b| (b as usize) % max_index_in_mnemonic)
            .collect::<IndexSet<_>>()
            .into_iter()
            .take(Self::NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM)
            .collect::<IndexSet<_>>();

        // always ask for the last word
        indices_in_mnemonic_of_words_to_confirm.insert(max_index_in_mnemonic);
        indices_in_mnemonic_of_words_to_confirm
            .into_iter()
            .sorted()
            .collect::<IndexSet<_>>()
    }

    /// Returns the `FactorSourceID` from the mnemonic with passphrase
    /// Panics if the mnemonic with passphrase wasn't yet created
    pub fn get_factor_source_id(&self) -> FactorSourceID {
        FactorSourceID::from(FactorSourceIDFromHash::new_for_device(
            &self.get_mnemonic_with_passphrase(),
        ))
    }

    /// Verifies if the `words_to_confirm` contains the expected number of words.
    /// Verifies if the `words_to_confirm` are correct within the previously created `MnemonicWithPassphrase`.
    /// Returns unconfirmed words if not all of the words were confirmed or the `MnemonicWithPassphrase`.
    pub fn build(
        &self,
        words_to_confirm: &HashMap<usize, String>,
    ) -> DeviceMnemonicBuildOutcome {
        if words_to_confirm.len()
            != Self::NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM
        {
            panic!("Words to confirm count mismatch");
        }

        let unconfirmed_indices_in_mnemonic = words_to_confirm
            .iter()
            .filter_map(|(&index, word)| {
                if !self.is_word_at_index_correct(word, index) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<IndexSet<_>>();

        if unconfirmed_indices_in_mnemonic.is_empty() {
            DeviceMnemonicBuildOutcome::Confirmed {
                mnemonic_with_passphrase: self.get_mnemonic_with_passphrase(),
            }
        } else {
            DeviceMnemonicBuildOutcome::Unconfirmed {
                indices_in_mnemonic: unconfirmed_indices_in_mnemonic,
            }
        }
    }

    /// Returns true if the mnemonic `word` at the given `index_in_mnemonic` is correct.
    fn is_word_at_index_correct(
        &self,
        word: impl AsRef<str>,
        index_in_mnemonic: usize,
    ) -> bool {
        self.get_mnemonic_with_passphrase()
            .mnemonic
            .words
            .get(index_in_mnemonic)
            .is_some_and(|w| w.word == word.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_common::prelude::indexset;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceMnemonicBuilder;

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
    #[allow(clippy::mutable_key_type)]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample(),
                SUT::sample(),
                SUT::sample_other(),
                SUT::sample_other(),
            ])
            .len(),
            2
        )
    }

    #[test]
    fn generate_new_mnemonic_with_passphrase() {
        let sut = SUT::default();
        pretty_assertions::assert_eq!(
            sut.mnemonic_with_passphrase.read().unwrap().clone(),
            None
        );
        sut.generate_new_mnemonic();
        assert!(sut.mnemonic_with_passphrase.read().unwrap().is_some());
    }

    #[test]
    fn create_mnemonic_with_passphrase_from_words_error() {
        let sut = SUT::default();

        let result = sut.create_mnemonic_from_words(vec![
            "abandon".to_owned(),
            "device".to_owned(),
            "word1".to_owned(),
            "word2".to_owned(),
            "word3".to_owned(),
            "sign".to_owned(),
        ]);

        pretty_assertions::assert_eq!(
            result,
            Err(CommonError::InvalidMnemonicWords {
                indices_in_mnemonic: vec![2, 3, 4]
                    .into_iter()
                    .collect::<Vec<_>>()
            })
        );
        assert!(sut.mnemonic_with_passphrase.read().unwrap().is_none());
    }

    #[test]
    fn create_mnemonic_with_passphrase_from_words() {
        let sut = SUT::default();
        let mnemonic = Mnemonic::sample();
        pretty_assertions::assert_eq!(
            sut.mnemonic_with_passphrase.read().unwrap().clone(),
            None
        );
        sut.create_mnemonic_from_words(
            mnemonic
                .words
                .iter()
                .map(|w| w.word.clone())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        assert!(sut.mnemonic_with_passphrase.read().unwrap().is_some());
    }

    #[test]
    fn get_indices_in_mnemonic_of_words_to_confirm() {
        let sut = SUT::sample();
        let indices = sut.get_indices_in_mnemonic_of_words_to_confirm();
        pretty_assertions::assert_eq!(indices.len(), 4);
        assert!(indices.contains(&23));
        pretty_assertions::assert_eq!(
            indices
                .clone()
                .into_iter()
                .sorted()
                .collect::<IndexSet<_>>(),
            indices
        );
    }

    #[test]
    #[should_panic(
        expected = "Mnemonic with passphrase should be created first"
    )]
    fn get_factor_source_id_panics_if_mnemonic_not_created() {
        let sut = SUT::default();
        let _ = sut.get_factor_source_id();
    }

    #[test]
    fn get_factor_source_id() {
        let sut = SUT::sample();
        let fsid = sut.get_factor_source_id();
        pretty_assertions::assert_eq!(
            fsid,
            FactorSourceID::from(FactorSourceIDFromHash::new_for_device(
                &MnemonicWithPassphrase::sample()
            ))
        );
    }

    #[test]
    fn get_words() {
        let sut = SUT::sample();
        let words = sut.get_words();
        pretty_assertions::assert_eq!(
            words,
            MnemonicWithPassphrase::sample().mnemonic.words
        );
    }

    #[test]
    #[should_panic(expected = "Words to confirm count mismatch")]
    fn build_with_words_to_confirm_count_mismatch() {
        let sut = SUT::sample();
        let words_to_confirm =
            vec![(0, "abandon".to_owned()), (1, "about".to_owned())]
                .into_iter()
                .collect::<HashMap<_, _>>();

        sut.build(&words_to_confirm);
    }

    #[test]
    fn build_with_all_valid_but_incorrect_words() {
        let sut = SUT::sample();
        let words_to_confirm = vec![
            (0, "abandon".to_owned()),
            (1, "about".to_owned()),
            (2, "device".to_owned()),
            (3, "sample".to_owned()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        pretty_assertions::assert_eq!(
            sut.build(&words_to_confirm),
            DeviceMnemonicBuildOutcome::Unconfirmed {
                indices_in_mnemonic: indexset![0, 1, 2, 3]
            }
        );
    }

    #[test]
    fn build_unconfirmed() {
        let sut = SUT::sample();
        let words_to_confirm = vec![
            (0, "device".to_owned()),
            (1, "phone".to_owned()),
            (7, "sign".to_owned()),
            (13, "invalid word".to_owned()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        pretty_assertions::assert_eq!(
            sut.build(&words_to_confirm),
            DeviceMnemonicBuildOutcome::Unconfirmed {
                indices_in_mnemonic: indexset![7, 13]
            }
        );
    }

    #[test]
    fn build_confirmed() {
        let sut = SUT::sample();
        let words_to_confirm = vec![
            (0, "device".to_owned()),
            (1, "phone".to_owned()),
            (2, "sign".to_owned()),
            (3, "source".to_owned()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        let result = sut.build(&words_to_confirm);

        pretty_assertions::assert_eq!(
            result,
            DeviceMnemonicBuildOutcome::Confirmed {
                mnemonic_with_passphrase: MnemonicWithPassphrase::sample()
            }
        );
    }
}

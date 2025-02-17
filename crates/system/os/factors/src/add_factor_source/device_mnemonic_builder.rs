use crate::prelude::*;

/// A builder of `MnemonicWithPassphrase` required for a new `DeviceFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug)]
pub struct DeviceMnemonicBuilder {
    mnemonic_with_passphrase: RwLock<Option<MnemonicWithPassphrase>>,
}

/// The result of the `DeviceMnemonicBuilder` representing the `build` function outcome,
/// before being able to use the underlying `MnemonicWithPassphrase`.
#[derive(Debug, PartialEq)]
pub enum DeviceMnemonicBuildResult {
    /// The mnemonic words were confirmed
    Confirmed {
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    },
    /// The mnemonic words were unconfirmed
    Unconfirmed { words: HashMap<usize, String> },
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
    pub fn new() -> Self {
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
    const NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM:
        usize = 3;
    const NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM: usize =
        Self::NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM
            + 1;

    pub fn get_indices_in_mnemonic_of_words_to_confirm(
        &self,
    ) -> IndexSet<usize> {
        let max_index_in_mnemonic =
            self.get_mnemonic_with_passphrase().mnemonic.words.len() - 1;
        let mut indices_in_mnemonic_of_words_to_confirm = generate_bytes::<1000>()
            .into_iter()
            .map(|b| (b as usize) % max_index_in_mnemonic)
            .take(Self::NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM)
            .collect::<IndexSet<_>>();

        // always ask for the last word
        indices_in_mnemonic_of_words_to_confirm.insert(max_index_in_mnemonic);
        indices_in_mnemonic_of_words_to_confirm
            .into_iter()
            .sorted()
            .collect::<IndexSet<_>>()
    }

    /// Verifies if the `words_to_confirm` contains the expected number of words.
    /// Verifies if the `words_to_confirm` are correct within the previously created `MnemonicWithPassphrase`.
    /// Returns unconfirmed words if not all of the words were confirmed or the `MnemonicWithPassphrase`.
    pub fn build(
        &self,
        words_to_confirm: &HashMap<usize, String>,
    ) -> DeviceMnemonicBuildResult {
        if words_to_confirm.len()
            != Self::NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM
        {
            return DeviceMnemonicBuildResult::Unconfirmed {
                words: words_to_confirm.clone(),
            };
        }

        let unconfirmed_words = words_to_confirm
            .iter()
            .filter_map(|(&index, word)| {
                if !self.is_word_at_index_correct(word, index) {
                    Some((index, word.clone()))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        if unconfirmed_words.is_empty() {
            DeviceMnemonicBuildResult::Confirmed {
                mnemonic_with_passphrase: self.get_mnemonic_with_passphrase(),
            }
        } else {
            DeviceMnemonicBuildResult::Unconfirmed {
                words: unconfirmed_words,
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
    pub fn create_new_mnemonic_with_passphrase(&self) -> &Self {
        let mnemonic = Mnemonic::generate_new();
        self.set_mnemonic_with_passphrase(mnemonic);
        self
    }

    /// Creates a new mnemonic from given `words`
    pub fn create_mnemonic_with_passphrase_from_words(
        &self,
        words: Vec<BIP39Word>,
    ) -> Result<&Self> {
        let mnemonic = Mnemonic::from_words(words)?;
        self.set_mnemonic_with_passphrase(mnemonic);
        Ok(self)
    }

    fn set_mnemonic_with_passphrase(&self, mnemonic: Mnemonic) {
        *self.mnemonic_with_passphrase.write().unwrap() =
            Some(MnemonicWithPassphrase::new(mnemonic));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn create_new_mnemonic_with_passphrase() {
        let sut = SUT::new();
        pretty_assertions::assert_eq!(
            sut.mnemonic_with_passphrase.read().unwrap().clone(),
            None
        );
        sut.create_new_mnemonic_with_passphrase();
        assert!(sut.mnemonic_with_passphrase.read().unwrap().is_some());
    }

    #[test]
    fn create_mnemonic_with_passphrase_from_words() {
        let sut = SUT::new();
        let mnemonic = Mnemonic::sample();
        pretty_assertions::assert_eq!(
            sut.mnemonic_with_passphrase.read().unwrap().clone(),
            None
        );
        sut.create_mnemonic_with_passphrase_from_words(mnemonic.words)
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
    fn build_with_not_enough_words() {
        let sut = SUT::sample();
        let words_to_confirm: HashMap<usize, String> =
            vec![(0, "abandon".into()), (1, "about".into())]
                .into_iter()
                .collect::<HashMap<_, _>>();

        pretty_assertions::assert_eq!(
            sut.build(&words_to_confirm),
            DeviceMnemonicBuildResult::Unconfirmed {
                words: words_to_confirm
            }
        );
    }

    #[test]
    fn build_with_all_valid_but_incorrect_words() {
        let sut = SUT::sample();
        let words_to_confirm: HashMap<usize, String> = vec![
            (0, "abandon".into()),
            (1, "about".into()),
            (2, "device".into()),
            (3, "sample".into()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        pretty_assertions::assert_eq!(
            sut.build(&words_to_confirm),
            DeviceMnemonicBuildResult::Unconfirmed {
                words: words_to_confirm
            }
        );
    }

    #[test]
    fn build_unconfirmed() {
        let sut = SUT::sample();
        let words_to_confirm: HashMap<usize, String> = vec![
            (0, "device".into()),
            (1, "phone".into()),
            (7, "sign".into()),
            (13, "invalid word".into()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        pretty_assertions::assert_eq!(
            sut.build(&words_to_confirm),
            DeviceMnemonicBuildResult::Unconfirmed {
                words: vec![(7, "sign".into()), (13, "invalid word".into()),]
                    .into_iter()
                    .collect()
            }
        );
    }

    #[test]
    fn build_confirmed() {
        let sut = SUT::sample();
        let words_to_confirm: HashMap<usize, String> = vec![
            (0, "device".into()),
            (1, "phone".into()),
            (2, "sign".into()),
            (3, "source".into()),
        ]
        .into_iter()
        .collect();

        let result = sut.build(&words_to_confirm);

        pretty_assertions::assert_eq!(
            result,
            DeviceMnemonicBuildResult::Confirmed {
                mnemonic_with_passphrase: MnemonicWithPassphrase::sample()
            }
        );
    }
}

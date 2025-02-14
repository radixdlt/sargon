use crate::prelude::*;

#[derive(Debug)]
pub struct DeviceFactorSourceBuilder {
    mnemonic_with_passphrase: RwLock<Option<MnemonicWithPassphrase>>,
}

impl Default for DeviceFactorSourceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for DeviceFactorSourceBuilder {
    fn eq(&self, other: &Self) -> bool {
        let mnemonic_with_passphrase =
            self.mnemonic_with_passphrase.read().unwrap();
        let other_mnemonic_with_passphrase =
            other.mnemonic_with_passphrase.read().unwrap();
        *mnemonic_with_passphrase == *other_mnemonic_with_passphrase
    }
}

impl Eq for DeviceFactorSourceBuilder {}

impl std::hash::Hash for DeviceFactorSourceBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mnemonic_with_passphrase =
            self.mnemonic_with_passphrase.read().unwrap();
        mnemonic_with_passphrase.hash(state);
    }
}

impl HasSampleValues for DeviceFactorSourceBuilder {
    fn sample() -> Self {
        Self::with_mnemonic_with_passphrase(MnemonicWithPassphrase::sample())
    }

    fn sample_other() -> Self {
        Self::with_mnemonic_with_passphrase(
            MnemonicWithPassphrase::sample_other(),
        )
    }
}

impl DeviceFactorSourceBuilder {
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
impl DeviceFactorSourceBuilder {
    const NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM:
        usize = 3;

    pub fn get_mnemonic_with_passphrase(&self) -> MnemonicWithPassphrase {
        self.mnemonic_with_passphrase
            .read()
            .unwrap()
            .clone()
            .expect("Mnemonic with passphrase should be created first")
    }

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

    /// Returns a map of unconfirmed words indices and their corresponding values
    /// or an empty map if all words are correct.
    pub fn get_unconfirmed_words(
        &self,
        words_to_confirm: &HashMap<usize, String>,
    ) -> HashMap<usize, String> {
        words_to_confirm
            .iter()
            .filter_map(|(&index, word)| {
                if !self.is_word_at_index_correct(word, index) {
                    Some((index, word.clone()))
                } else {
                    None
                }
            })
            .collect()
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

// ====================
// ===== MUTATION =====
// ====================
impl DeviceFactorSourceBuilder {
    pub fn create_new_mnemonic_with_passphrase(&self) -> &Self {
        let mnemonic = Mnemonic::generate_new();
        self.set_mnemonic_with_passphrase(mnemonic);
        self
    }

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
    type SUT = DeviceFactorSourceBuilder;

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
    fn get_indices_in_mnemonic_of_words_to_confirm() {
        let sut = SUT::sample();
        let indices = sut.get_indices_in_mnemonic_of_words_to_confirm();
        assert_eq!(indices.len(), 4);
        assert!(indices.contains(&23));
        assert_eq!(
            indices
                .clone()
                .into_iter()
                .sorted()
                .collect::<IndexSet<_>>(),
            indices
        );
        println!("{:?}", indices);
    }

    #[test]
    fn get_unconfirmed_words() {
        let sut = SUT::sample();

        let words_to_confirm: HashMap<usize, String> = vec![
            (0, "abandon".into()),
            (1, "about".into()),
            (2, "device".into()),
            (3, "sample".into()),
        ]
        .into_iter()
        .collect();

        let unconfirmed_words = sut.get_unconfirmed_words(&words_to_confirm);

        assert_eq!(unconfirmed_words, words_to_confirm);
    }
}

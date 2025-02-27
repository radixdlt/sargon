use crate::prelude::*;

/// A builder of `MnemonicWithPassphrase` required for a new `OffDeviceMnemonicFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug)]
pub struct OffDeviceMnemonicBuilder {
    mnemonic_with_passphrase: RwLock<Option<MnemonicWithPassphrase>>,
}

impl Default for OffDeviceMnemonicBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for OffDeviceMnemonicBuilder {
    fn eq(&self, other: &Self) -> bool {
        let mnemonic_with_passphrase =
            self.mnemonic_with_passphrase.read().unwrap();
        let other_mnemonic_with_passphrase =
            other.mnemonic_with_passphrase.read().unwrap();
        *mnemonic_with_passphrase == *other_mnemonic_with_passphrase
    }
}

impl Eq for OffDeviceMnemonicBuilder {}

impl std::hash::Hash for OffDeviceMnemonicBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mnemonic_with_passphrase =
            self.mnemonic_with_passphrase.read().unwrap();
        mnemonic_with_passphrase.hash(state);
    }
}

impl HasSampleValues for OffDeviceMnemonicBuilder {
    fn sample() -> Self {
        Self::with_mnemonic_with_passphrase(
            MnemonicWithPassphrase::sample_off_device(),
        )
    }

    fn sample_other() -> Self {
        Self::with_mnemonic_with_passphrase(
            MnemonicWithPassphrase::sample_off_device_other(),
        )
    }
}

impl OffDeviceMnemonicBuilder {
    fn new() -> Self {
        Self {
            mnemonic_with_passphrase: RwLock::new(None),
        }
    }

    fn with_mnemonic_with_passphrase(
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
impl OffDeviceMnemonicBuilder {
    /// Returns the words of the mnemonic.
    pub fn get_words(&self) -> Vec<BIP39Word> {
        self.get_mnemonic_with_passphrase().mnemonic.words
    }

    /// Returns the word count of the mnemonic.
    pub fn get_word_count(&self) -> BIP39WordCount {
        self.get_mnemonic_with_passphrase().mnemonic.word_count
    }

    /// Returns the `mnemonic_with_passphrase` if it was previously created or panics.
    fn get_mnemonic_with_passphrase(&self) -> MnemonicWithPassphrase {
        self.mnemonic_with_passphrase
            .read()
            .unwrap()
            .clone()
            .expect("Mnemonic should be created first")
    }
}

// ====================
// ===== MUTATION =====
// ====================
impl OffDeviceMnemonicBuilder {
    /// Generates a new mnemonic with specified `word_count` and sets it as the `mnemonic_with_passphrase`.
    pub fn generate_new_mnemonic(&self, word_count: BIP39WordCount) -> &Self {
        let mnemonic = Mnemonic::generate_new_with_word_count(word_count);
        self.set_mnemonic_with_passphrase(mnemonic);
        self
    }

    /// Creates a new mnemonic from given `words`
    pub fn create_mnemonic(&self, words: Vec<String>) -> Result<&Self> {
        let mnemonic =
            CommonMnemonicBuilder::create_mnemonic_from_words(words)?;
        self.set_mnemonic_with_passphrase(mnemonic);
        Ok(self)
    }

    fn set_mnemonic_with_passphrase(&self, mnemonic: Mnemonic) {
        *self.mnemonic_with_passphrase.write().unwrap() =
            Some(MnemonicWithPassphrase::new(mnemonic));
    }
}

impl OffDeviceMnemonicBuilder {
    /// Returns the `FactorSourceID` from the mnemonic
    /// Panics if the mnemonic wasn't yet created
    pub fn get_factor_source_id(&self) -> FactorSourceID {
        FactorSourceID::from(FactorSourceIDFromHash::new_for_off_device(
            &self.get_mnemonic_with_passphrase(),
        ))
    }

    /// This doesn't actually build anything, it just returns the `MnemonicWithPassphrase`.
    /// Kept the name for consistency with other builders.
    pub fn build(&self) -> MnemonicWithPassphrase {
        self.get_mnemonic_with_passphrase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OffDeviceMnemonicBuilder;

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
    fn generate_new_mnemonic() {
        let test = |word_count| {
            let sut = SUT::default();
            pretty_assertions::assert_eq!(
                sut.mnemonic_with_passphrase.read().unwrap().clone(),
                None
            );
            sut.generate_new_mnemonic(word_count);

            let result = sut
                .mnemonic_with_passphrase
                .read()
                .unwrap()
                .clone()
                .unwrap();
            pretty_assertions::assert_eq!(
                result.mnemonic.word_count,
                word_count
            );
        };

        BIP39WordCount::all().iter().for_each(|&wc| test(wc));
    }

    #[test]
    fn create_mnemonic_error() {
        let sut = SUT::default();

        let result = sut.create_mnemonic(vec![
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
    fn create_mnemonic() {
        let test = |words: Vec<BIP39Word>| {
            let sut = SUT::default();
            pretty_assertions::assert_eq!(
                sut.mnemonic_with_passphrase.read().unwrap().clone(),
                None
            );
            sut.create_mnemonic(
                words.iter().map(|w| w.word.clone()).collect::<Vec<_>>(),
            )
            .unwrap();

            pretty_assertions::assert_eq!(
                sut.mnemonic_with_passphrase
                    .read()
                    .unwrap()
                    .clone()
                    .unwrap()
                    .mnemonic
                    .words,
                words
            );
        };

        BIP39WordCount::all().iter().for_each(|&wc| {
            test(Mnemonic::generate_new_with_word_count(wc).words)
        });
    }

    #[test]
    #[should_panic(expected = "Mnemonic should be created first")]
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
            FactorSourceID::from(FactorSourceIDFromHash::new_for_off_device(
                &MnemonicWithPassphrase::sample_off_device()
            ))
        );
    }

    #[test]
    fn get_words() {
        let sut = SUT::sample();
        let words = sut.get_words();
        pretty_assertions::assert_eq!(
            words,
            MnemonicWithPassphrase::sample_off_device().mnemonic.words
        );
    }

    #[test]
    fn build() {
        let sut = SUT::sample();

        let result = sut.build();

        pretty_assertions::assert_eq!(
            result,
            MnemonicWithPassphrase::sample_off_device()
        );
    }
}

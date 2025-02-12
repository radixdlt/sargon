use crate::prelude::*;
use rand::{rngs::OsRng, Rng};

pub struct DeviceFactorAddingManager {
    os_ref: Arc<dyn OsNewFactorAdding>,
    factor_identification: RwLock<Option<FactorIdentification>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FactorIdentification {
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    factor_source: DeviceFactorSource,
    mnemonic_words: Vec<BIP39Word>,
    confirmation_indices: Vec<u8>,
}

impl std::hash::Hash for FactorIdentification {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.mnemonic_with_passphrase.hash(state);
        self.factor_source.hash(state);
        self.mnemonic_words.hash(state);
        for index in &self.confirmation_indices {
            index.hash(state);
        }
    }
}

impl PartialEq for DeviceFactorAddingManager {
    fn eq(&self, other: &Self) -> bool {
        let factor_identification = self
            .factor_identification
            .read()
            .expect("Failed to read factor identification");
        let other_factor_identification = other
            .factor_identification
            .read()
            .expect("Failed to read other factor identification");

        *factor_identification == *other_factor_identification
    }
}

impl Eq for DeviceFactorAddingManager {}

impl Clone for DeviceFactorAddingManager {
    fn clone(&self) -> Self {
        Self {
            os_ref: self.os_ref.clone(),
            factor_identification: RwLock::new(
                self.factor_identification
                    .read()
                    .expect("Failed to read factor identification")
                    .clone(),
            ),
        }
    }
}

impl std::hash::Hash for DeviceFactorAddingManager {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let factor_identification = self
            .factor_identification
            .read()
            .expect("Failed to read factor identification");
        factor_identification.hash(state);
    }
}

impl FactorIdentification {
    const NUMBER_OF_CONFIRMATION_WORDS: u8 = 4;

    fn new(mnemonic: Mnemonic, host_info: HostInfo) -> Self {
        let mnemonic_words = mnemonic.clone().words;
        let number_of_words = mnemonic_words.clone().len() as u8;
        let mnemonic_with_passphrase =
            MnemonicWithPassphrase::new(mnemonic.clone());
        let factor_source = DeviceFactorSource::babylon(
            false,
            &mnemonic_with_passphrase,
            &host_info,
        );

        Self {
            mnemonic_with_passphrase,
            factor_source,
            mnemonic_words,
            confirmation_indices: Self::generate_confirmation_indices(
                number_of_words,
            ),
        }
    }

    fn generate_confirmation_indices(number_of_words: u8) -> Vec<u8> {
        let max_confirmation_index = number_of_words - 1;
        let mut csprng = OsRng;
        let mut confirmation_indices: HashSet<u8> = HashSet::new();

        while (confirmation_indices.len() as u8)
            < Self::NUMBER_OF_CONFIRMATION_WORDS - 1
        {
            confirmation_indices
                .insert(csprng.gen::<u8>() % max_confirmation_index);
        }

        // always ask for the last word
        confirmation_indices.insert(max_confirmation_index);
        confirmation_indices
            .into_iter()
            .sorted()
            .collect::<Vec<u8>>()
    }
}

impl DeviceFactorAddingManager {
    pub fn new(sargon_os: Arc<dyn OsNewFactorAdding>) -> Self {
        Self {
            os_ref: sargon_os,
            factor_identification: RwLock::new(None),
        }
    }
}

// ====================
// ==== GET / READ ====
// ====================
impl DeviceFactorAddingManager {
    pub fn get_factor_source(&self) -> FactorSource {
        self.get_factor_identification().factor_source.into()
    }

    pub fn get_mnemonic_words(&self) -> Vec<BIP39Word> {
        self.get_factor_identification().mnemonic_words
    }

    pub fn get_confirmation_indices(&self) -> Vec<u8> {
        self.get_factor_identification().confirmation_indices
    }

    fn get_factor_identification(&self) -> FactorIdentification {
        self.factor_identification
            .read()
            .unwrap()
            .clone()
            .expect("Factor identification not initialized")
    }
}

// ====================
// ===== MUTATION =====
// ====================
impl DeviceFactorAddingManager {
    pub fn set_factor_name(&self, name: impl AsRef<str>) -> &Self {
        self.factor_identification
            .write()
            .unwrap()
            .as_mut()
            .expect("Factor identification not initialized")
            .factor_source
            .set_name(name.as_ref().to_owned());
        self
    }

    pub fn create_new_factor_source(&self, host_info: HostInfo) -> &Self {
        let mnemonic = Mnemonic::generate_new();
        self.create_factor_source(mnemonic, host_info);
        self
    }

    pub fn create_factor_source_from_mnemonic_words(
        &self,
        host_info: HostInfo,
        words: Vec<BIP39Word>,
    ) -> Result<&Self> {
        let mnemonic = Mnemonic::from_words(words)?;
        self.create_factor_source(mnemonic, host_info);
        Ok(self)
    }

    fn create_factor_source(&self, mnemonic: Mnemonic, host_info: HostInfo) {
        *self.factor_identification.write().unwrap() =
            Some(FactorIdentification::new(mnemonic, host_info))
    }
}

impl DeviceFactorAddingManager {
    pub fn is_word_at_index_correct(
        &self,
        word: impl AsRef<str>,
        index: u8,
    ) -> bool {
        self.get_mnemonic_words()
            .get(index as usize)
            .is_some_and(|w| w.word == word.as_ref())
    }

    /// Returns a map of incorrect confirmation words indices and their corresponding incorrect words
    /// or an empty map if all confirmation words are correct.
    pub fn get_incorrect_confirmation_words(
        &self,
        words_to_confirm: &HashMap<u8, String>,
    ) -> HashMap<u8, String> {
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

    pub async fn resolve_host_info(&self) -> HostInfo {
        self.os_ref.resolve_host_info().await
    }

    pub async fn is_factor_already_in_use(&self) -> Result<bool> {
        self.os_ref
            .is_factor_already_in_use(self.get_factor_source())
            .await
    }

    pub async fn add_factor(&self) -> Result<()> {
        self.os_ref
            .add_new_factor(
                self.get_factor_source(),
                self.get_factor_identification().mnemonic_with_passphrase,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceFactorAddingManager;

    struct MockOsNewFactorAdding {
        stubbed_factor_already_in_use: Result<bool>,
        stubbed_add_new_factor_result: Result<()>,
        stubbed_host_info: HostInfo,
    }

    impl MockOsNewFactorAdding {
        fn with_ok_factor_already_in_use(factor_already_in_use: bool) -> Self {
            Self {
                stubbed_factor_already_in_use: Ok(factor_already_in_use),
                stubbed_add_new_factor_result: Ok(()),
                stubbed_host_info: HostInfo::sample(),
            }
        }
    }

    #[async_trait::async_trait]
    impl OsNewFactorAdding for MockOsNewFactorAdding {
        async fn is_factor_already_in_use(
            &self,
            _factor_source: FactorSource,
        ) -> Result<bool> {
            self.stubbed_factor_already_in_use.clone()
        }

        async fn add_new_factor(
            &self,
            _factor_source: FactorSource,
            _mnemonic_with_passphrase: MnemonicWithPassphrase,
        ) -> Result<()> {
            self.stubbed_add_new_factor_result.clone()
        }

        async fn resolve_host_info(&self) -> HostInfo {
            self.stubbed_host_info.clone()
        }
    }

    #[test]
    fn generate_confirmation_indices() {
        let indices = FactorIdentification::generate_confirmation_indices(24);
        assert_eq!(indices.len(), 4);
        assert!(indices.contains(&23))
    }

    #[actix_rt::test]
    async fn get_incorrect_confirmation_words() {
        let os_ref =
            MockOsNewFactorAdding::with_ok_factor_already_in_use(false);
        let sut = SUT::new(Arc::new(os_ref));
        let mnemonic = Mnemonic::sample();
        let host_info = HostInfo::sample();

        sut.create_factor_source(mnemonic, host_info);

        let words_to_confirm: HashMap<u8, String> = vec![
            (0, "word1".into()),
            (1, "word2".into()),
            (2, "word3".into()),
            (3, "word4".into()),
        ]
        .into_iter()
        .collect();

        let incorrect_words =
            sut.get_incorrect_confirmation_words(&words_to_confirm);

        assert_eq!(incorrect_words, words_to_confirm);
    }
}

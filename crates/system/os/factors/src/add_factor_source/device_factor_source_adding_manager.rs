use crate::prelude::*;

pub struct DeviceFactorSourceAddingManager {
    factor_adder: Arc<dyn OsFactorSourceAdder>,
    factor_source_identification: RwLock<Option<FactorSourceIdentification>>,
    factor_source_name: RwLock<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct FactorSourceIdentification {
    mnemonic_with_passphrase: MnemonicWithPassphrase,
}

impl PartialEq for DeviceFactorSourceAddingManager {
    fn eq(&self, other: &Self) -> bool {
        let factor_source_identification = self
            .factor_source_identification
            .read()
            .expect("Failed to read factor source identification");
        let other_factor_source_identification = other
            .factor_source_identification
            .read()
            .expect("Failed to read other factor source identification");
        *factor_source_identification == *other_factor_source_identification
    }
}

impl Eq for DeviceFactorSourceAddingManager {}

impl std::hash::Hash for DeviceFactorSourceAddingManager {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let factor_source_identification = self
            .factor_source_identification
            .read()
            .expect("Failed to read factor source identification");
        factor_source_identification.hash(state);
    }
}

impl Debug for DeviceFactorSourceAddingManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let factor_source_identification = self
            .factor_source_identification
            .read()
            .expect("Failed to read factor source identification");
        write!(f, "{:?}", factor_source_identification)
    }
}

impl FactorSourceIdentification {
    const NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM:
        usize = 3;

    fn new(mnemonic: Mnemonic) -> Self {
        let mnemonic_with_passphrase =
            MnemonicWithPassphrase::new(mnemonic.clone());
        Self {
            mnemonic_with_passphrase,
        }
    }

    fn generate_indices_in_mnemonic_of_words_to_confirm(
        &self,
    ) -> IndexSet<usize> {
        let max_index_in_mnemonic =
            self.mnemonic_with_passphrase.mnemonic.words.len() - 1;
        let mut indices_in_mnemonic_of_words_to_confirm = generate_bytes::<1000>()
            .into_iter()
            .map(|b| b as usize)
            .take(Self::NUMBER_OF_WORDS_OF_MNEMONIC_USER_NEED_TO_CONFIRM_EXCL_CHECKSUM)
            .collect::<IndexSet<_>>();

        // always ask for the last word
        indices_in_mnemonic_of_words_to_confirm.insert(max_index_in_mnemonic);
        indices_in_mnemonic_of_words_to_confirm
            .into_iter()
            .sorted()
            .collect::<IndexSet<_>>()
    }
}

impl HasSampleValues for FactorSourceIdentification {
    fn sample() -> Self {
        FactorSourceIdentification::new(Mnemonic::sample())
    }

    fn sample_other() -> Self {
        FactorSourceIdentification::new(Mnemonic::sample_other())
    }
}

impl HasSampleValues for DeviceFactorSourceAddingManager {
    fn sample() -> Self {
        DeviceFactorSourceAddingManager::with_details(
            Arc::new(MockOsFactorAdder::new()),
            Some(FactorSourceIdentification::sample()),
            "First",
        )
    }

    fn sample_other() -> Self {
        DeviceFactorSourceAddingManager::with_details(
            Arc::new(MockOsFactorAdder::new()),
            Some(FactorSourceIdentification::sample_other()),
            "Second",
        )
    }
}

impl DeviceFactorSourceAddingManager {
    pub fn new(os_ref: Arc<dyn OsFactorSourceAdder>) -> Self {
        Self::with_details(os_ref, None, "")
    }

    fn with_details(
        os_ref: Arc<dyn OsFactorSourceAdder>,
        factor_source_identification: Option<FactorSourceIdentification>,
        factor_source_name: impl AsRef<str>,
    ) -> Self {
        Self {
            factor_adder: os_ref,
            factor_source_identification: RwLock::new(
                factor_source_identification,
            ),
            factor_source_name: RwLock::new(
                factor_source_name.as_ref().to_owned(),
            ),
        }
    }
}

// ====================
// ==== GET / READ ====
// ====================
impl DeviceFactorSourceAddingManager {
    pub fn get_mnemonic_words(&self) -> Vec<BIP39Word> {
        self.get_factor_source_identification()
            .mnemonic_with_passphrase
            .mnemonic
            .words
    }

    pub fn get_indices_in_mnemonic_of_words_to_confirm(
        &self,
    ) -> IndexSet<usize> {
        self.get_factor_source_identification()
            .generate_indices_in_mnemonic_of_words_to_confirm()
    }

    fn get_factor_source_identification(&self) -> FactorSourceIdentification {
        self.factor_source_identification
            .read()
            .unwrap()
            .clone()
            .expect("Factor source identification should be initialized")
    }

    pub fn get_factor_source_name(&self) -> String {
        self.factor_source_name.read().unwrap().clone()
    }
}

// ====================
// ===== MUTATION =====
// ====================
impl DeviceFactorSourceAddingManager {
    pub fn set_factor_source_name(&self, name: impl AsRef<str>) -> &Self {
        *self.factor_source_name.write().unwrap() = name.as_ref().to_owned();
        self
    }

    pub async fn create_new_mnemonic(&self) -> &Self {
        let mnemonic = Mnemonic::generate_new();
        self.set_mnemonic(mnemonic).await;
        self
    }

    pub async fn create_mnemonic_from_words(
        &self,
        words: Vec<BIP39Word>,
    ) -> Result<&Self> {
        let mnemonic = Mnemonic::from_words(words)?;
        self.set_mnemonic(mnemonic).await;
        Ok(self)
    }

    async fn set_mnemonic(&self, mnemonic: Mnemonic) {
        *self.factor_source_identification.write().unwrap() =
            Some(FactorSourceIdentification::new(mnemonic))
    }
}

impl DeviceFactorSourceAddingManager {
    /// Returns true if the mnemonic `word` at the given `index_in_mnemonic` is correct.
    pub fn is_word_at_index_correct(
        &self,
        word: impl AsRef<str>,
        index_in_mnemonic: usize,
    ) -> bool {
        self.get_mnemonic_words()
            .get(index_in_mnemonic)
            .is_some_and(|w| w.word == word.as_ref())
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

    /// Checks if profile already contains a factor source with the same `FactorSourceID`.
    pub async fn is_factor_source_already_in_use(&self) -> Result<bool> {
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            &self
                .get_factor_source_identification()
                .mnemonic_with_passphrase,
        );
        self.factor_adder
            .is_factor_source_already_in_use(FactorSourceID::from(id))
            .await
    }

    /// Adds the factor source
    pub async fn add_factor_source(&self) -> Result<()> {
        let display_name = DisplayName::new(self.get_factor_source_name())
            .map_err(|e| {
                error!("Invalid DisplayName {:?}", e);
                CommonError::FactorSourceNameInvalid
            })?;

        self.factor_adder
            .add_new_factor_source(
                self.get_factor_source_identification()
                    .mnemonic_with_passphrase,
                display_name,
            )
            .await
    }
}

struct MockOsFactorAdder {
    stubbed_factor_source_already_in_use: Result<bool>,
    stubbed_add_factor_source_result: Result<()>,
}

impl MockOsFactorAdder {
    fn new() -> Self {
        Self::with_ok_factor_source_already_in_use(false)
    }

    fn with_ok_factor_source_already_in_use(
        factor_source_already_in_use: bool,
    ) -> Self {
        Self {
            stubbed_factor_source_already_in_use: Ok(
                factor_source_already_in_use,
            ),
            stubbed_add_factor_source_result: Ok(()),
        }
    }
}

#[async_trait::async_trait]
impl OsFactorSourceAdder for MockOsFactorAdder {
    async fn is_factor_source_already_in_use(
        &self,
        _factor_source_id: FactorSourceID,
    ) -> Result<bool> {
        self.stubbed_factor_source_already_in_use.clone()
    }

    async fn add_new_factor_source(
        &self,
        _mnemonic_with_passphrase: MnemonicWithPassphrase,
        _name: DisplayName,
    ) -> Result<()> {
        self.stubbed_add_factor_source_result.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceFactorSourceAddingManager;

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
        )
    }

    #[actix_rt::test]
    async fn get_unconfirmed_words() {
        let sut = SUT::sample();

        let words_to_confirm: HashMap<usize, String> = vec![
            (0, "word1".into()),
            (1, "word2".into()),
            (2, "word3".into()),
            (3, "word4".into()),
        ]
        .into_iter()
        .collect();

        let unconfirmed_words = sut.get_unconfirmed_words(&words_to_confirm);

        assert_eq!(unconfirmed_words, words_to_confirm);
    }
}

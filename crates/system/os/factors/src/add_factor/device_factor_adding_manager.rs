use crate::prelude::*;
use rand::{rngs::OsRng, Rng};

pub struct DeviceFactorAddingManager {
    sargon_os: Arc<SargonOS>,
    factor_identification: RwLock<Option<FactorIdentification>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FactorIdentification {
    factor_source: DeviceFactorSource,
    mnemonic_words: Vec<BIP39Word>,
    confirmation_indices: HashSet<u8>,
}

impl std::hash::Hash for FactorIdentification {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
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
            sargon_os: self.sargon_os.clone(),
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

    async fn new(
        sargon_os: &Arc<SargonOS>,
        mnemonic: Mnemonic,
    ) -> Result<Self> {
        let mnemonic_words = mnemonic.clone().words;
        let number_of_words = mnemonic_words.clone().len() as u8;
        let factor_source = sargon_os
            .create_device_factor_source(
                MnemonicWithPassphrase::new(mnemonic),
                DeviceFactorSourceType::Babylon { is_main: false },
            )
            .await?;

        Ok(Self {
            factor_source,
            mnemonic_words,
            confirmation_indices: Self::generate_confirmation_indices(
                number_of_words,
            ),
        })
    }

    fn generate_confirmation_indices(number_of_words: u8) -> HashSet<u8> {
        let max_confirmation_index = number_of_words - 1;
        let mut csprng = OsRng;
        let mut confirmation_indices: HashSet<u8> = HashSet::new();

        while (confirmation_indices.len() as u8)
            < Self::NUMBER_OF_CONFIRMATION_WORDS - 1
        {
            confirmation_indices
                .insert(csprng.gen::<u8>() % max_confirmation_index);
        }

        // always ask for last word
        confirmation_indices.insert(max_confirmation_index);
        confirmation_indices
    }
}

impl DeviceFactorAddingManager {
    pub fn new(sargon_os: Arc<SargonOS>) -> Self {
        Self {
            sargon_os,
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

    pub fn get_confirmation_indices(&self) -> HashSet<u8> {
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
}

impl DeviceFactorAddingManager {
    pub async fn create_new_factor_source(&self) -> Result<&Self> {
        let mnemonic = Mnemonic::generate_new();
        self.create_factor_source(mnemonic).await?;
        Ok(self)
    }

    pub async fn create_factor_source_from_mnemonic_words(
        &self,
        words: Vec<BIP39Word>,
    ) -> Result<&Self> {
        let mnemonic = Mnemonic::from_words(words)?;
        self.create_factor_source(mnemonic).await?;
        Ok(self)
    }

    pub fn mnemonic_words_match(&self, words: Vec<BIP39Word>) -> bool {
        self.get_mnemonic_words() == words
    }

    async fn create_factor_source(&self, mnemonic: Mnemonic) -> Result<()> {
        *self.factor_identification.write().unwrap() =
            Some(FactorIdentification::new(&self.sargon_os, mnemonic).await?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceFactorAddingManager;

    #[test]
    fn generate_confirmation_indices() {
        let indices = FactorIdentification::generate_confirmation_indices(24);
        assert_eq!(indices.len(), 4);
        assert!(indices.contains(&23))
    }
}

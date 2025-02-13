use crate::prelude::*;
use sargon::DeviceFactorSourceAddingManager as InternalDeviceFactorSourceAddingManager;
use sargon::{OsFactorSourceAdder, OsFactorSourceAddingManagerFactory};

#[derive(Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct DeviceFactorSourceAddingManager {
    wrapped: Arc<InternalDeviceFactorSourceAddingManager>,
}

#[uniffi::export]
impl SargonOS {
    pub fn make_device_factor_source_adding_manager(
        &self,
    ) -> DeviceFactorSourceAddingManager {
        let internal = self.wrapped.make_device_factor_source_adding_manager();
        DeviceFactorSourceAddingManager::new(internal)
    }
}

impl DeviceFactorSourceAddingManager {
    pub fn new(internal: InternalDeviceFactorSourceAddingManager) -> Self {
        Self {
            wrapped: Arc::new(internal),
        }
    }
}

impl DeviceFactorSourceAddingManager {
    fn get<R>(
        &self,
        access: impl Fn(&InternalDeviceFactorSourceAddingManager) -> R,
    ) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalDeviceFactorSourceAddingManager>,
        ) -> &InternalDeviceFactorSourceAddingManager,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            _ = write(&builder.wrapped);
        })
    }

    async fn _create_new_factor_source(
        manager: Arc<InternalDeviceFactorSourceAddingManager>,
    ) -> Result<()> {
        manager.create_new_factor_source().await;
        Ok(())
    }

    async fn _create_factor_source_from_mnemonic_words(
        manager: Arc<InternalDeviceFactorSourceAddingManager>,
        words: Vec<BIP39Word>,
    ) -> Result<()> {
        manager
            .create_factor_source_from_mnemonic_words(
                words.clone().into_internal(),
            )
            .await
            .map(|_| ())
            .into_result()
    }
}

// ====================
// ==== GET / READ ====
// ====================
#[uniffi::export]
impl DeviceFactorSourceAddingManager {
    pub fn get_mnemonic_words(self: Arc<Self>) -> Vec<BIP39Word> {
        self.get(|manager| {
            manager
                .get_mnemonic_words()
                .into_iter()
                .map(|internal| internal.into())
                .collect::<Vec<BIP39Word>>()
        })
    }

    pub fn get_indices_in_mnemonic_of_words_to_confirm(
        self: Arc<Self>,
    ) -> Vec<u8> {
        self.get(|manager| {
            manager
                .get_indices_in_mnemonic_of_words_to_confirm()
                .into_iter()
                .map(|i| i as u8)
                .collect::<Vec<_>>()
        })
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl DeviceFactorSourceAddingManager {
    pub fn set_factor_source_name(
        self: Arc<Self>,
        name: DisplayName,
    ) -> Arc<Self> {
        self.set(|manager| manager.set_factor_name(name.into_internal()))
    }

    pub async fn create_new_factor_source(
        self: Arc<Self>,
    ) -> Result<Arc<Self>> {
        builder_arc_map_future_result(self, |manager| {
            Self::_create_new_factor_source(manager.wrapped.clone())
        })
        .await
    }

    pub async fn create_factor_source_from_mnemonic_words(
        self: Arc<Self>,
        words: Vec<BIP39Word>,
    ) -> Result<Arc<Self>> {
        builder_arc_map_future_result(self, |manager| {
            Self::_create_factor_source_from_mnemonic_words(
                manager.wrapped.clone(),
                words,
            )
        })
        .await
    }
}

#[uniffi::export]
impl DeviceFactorSourceAddingManager {
    pub fn is_word_at_index_correct(
        self: Arc<Self>,
        word: String,
        index: u8,
    ) -> bool {
        self.wrapped.is_word_at_index_correct(word, index as usize)
    }

    pub fn get_unconfirmed_words(
        self: Arc<Self>,
        words_to_confirm: HashMap<u8, String>,
    ) -> HashMap<u8, String> {
        self.wrapped
            .get_unconfirmed_words(
                &words_to_confirm
                    .into_iter()
                    .map(|(k, v)| (k as usize, v))
                    .collect::<HashMap<_, _>>(),
            )
            .into_iter()
            .map(|(k, v)| (k as u8, v))
            .collect::<HashMap<_, _>>()
    }

    pub async fn is_factor_source_already_in_use(&self) -> Result<bool> {
        self.wrapped
            .is_factor_source_already_in_use()
            .await
            .into_result()
    }

    pub async fn add_factor_source(&self) -> Result<()> {
        self.wrapped.add_factor_source().await.into_result()
    }
}

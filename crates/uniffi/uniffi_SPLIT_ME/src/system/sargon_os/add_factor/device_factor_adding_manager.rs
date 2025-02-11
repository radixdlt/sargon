use crate::prelude::*;
use sargon::OsNewFactorAdding;

#[derive(Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct DeviceFactorAddingManager {
    wrapped: Arc<sargon::DeviceFactorAddingManager>,
}

#[uniffi::export]
impl SargonOS {
    pub fn make_device_factor_adding_manager(
        &self,
    ) -> DeviceFactorAddingManager {
        let manager = self.wrapped.make_device_factor_adding_manager();
        DeviceFactorAddingManager::new(manager)
    }
}

impl DeviceFactorAddingManager {
    pub fn new(internal: sargon::DeviceFactorAddingManager) -> Self {
        Self {
            wrapped: Arc::new(internal),
        }
    }
}

impl DeviceFactorAddingManager {
    fn get<R>(
        &self,
        access: impl Fn(&sargon::DeviceFactorAddingManager) -> R,
    ) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<sargon::DeviceFactorAddingManager>,
        ) -> &sargon::DeviceFactorAddingManager,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            _ = write(&builder.wrapped);
        })
    }

    fn set_from_result(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<sargon::DeviceFactorAddingManager>,
        ) -> Result<&sargon::DeviceFactorAddingManager>,
    ) -> Result<Arc<Self>> {
        builder_arc_map_result(self, |builder| {
            write(&builder.wrapped).map(|_| ())
        })
    }
}

// ====================
// ==== GET / READ ====
// ====================
#[uniffi::export]
impl DeviceFactorAddingManager {
    pub fn get_factor_source(self: Arc<Self>) -> FactorSource {
        self.get(|manager| manager.get_factor_source().into())
    }

    pub fn get_mnemonic_words(self: Arc<Self>) -> Vec<BIP39Word> {
        self.get(|manager| {
            manager
                .get_mnemonic_words()
                .into_iter()
                .map(|internal| internal.into())
                .collect::<Vec<BIP39Word>>()
        })
    }

    pub fn get_confirmation_indices(self: Arc<Self>) -> Vec<u8> {
        self.get(|manager| {
            manager
                .get_confirmation_indices()
                .into_iter()
                .collect::<Vec<u8>>()
        })
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl DeviceFactorAddingManager {
    pub fn set_factor_name(self: Arc<Self>, name: String) -> Arc<Self> {
        self.set(|manager| manager.set_factor_name(&name))
    }

    pub async fn create_new_factor_source(self: Arc<Self>) -> Arc<Self> {
        let host_info = self.wrapped.resolve_host_info().await;
        self.set(|manager| manager.create_new_factor_source(host_info.clone()))
    }

    pub async fn create_factor_source_from_mnemonic_words(
        self: Arc<Self>,
        words: Vec<BIP39Word>,
    ) -> Result<Arc<Self>> {
        let host_info = self.wrapped.resolve_host_info().await;
        self.set_from_result(|manager| {
            manager
                .create_factor_source_from_mnemonic_words(
                    host_info.clone(),
                    words.clone().into_internal(),
                )
                .into_result()
        })
    }
}

#[uniffi::export]
impl DeviceFactorAddingManager {
    pub fn mnemonic_words_match(
        self: Arc<Self>,
        words: Vec<BIP39Word>,
    ) -> bool {
        self.wrapped.mnemonic_words_match(words.into_internal())
    }
}

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
}

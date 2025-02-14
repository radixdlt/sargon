use crate::prelude::*;
use sargon::DeviceFactorSourceBuilder as InternalDeviceFactorSourceBuilder;

#[derive(Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct DeviceFactorSourceBuilder {
    wrapped: Arc<InternalDeviceFactorSourceBuilder>,
}

#[uniffi::export]
impl DeviceFactorSourceBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wrapped: Arc::new(InternalDeviceFactorSourceBuilder::new()),
        })
    }
}

impl DeviceFactorSourceBuilder {
    fn get<R>(
        &self,
        access: impl Fn(&InternalDeviceFactorSourceBuilder) -> R,
    ) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalDeviceFactorSourceBuilder>,
        ) -> &InternalDeviceFactorSourceBuilder,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            _ = write(&builder.wrapped);
        })
    }

    fn set_from_result(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalDeviceFactorSourceBuilder>,
        ) -> Result<&InternalDeviceFactorSourceBuilder>,
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
impl DeviceFactorSourceBuilder {
    pub fn get_mnemonic_with_passphrase(
        self: Arc<Self>,
    ) -> MnemonicWithPassphrase {
        self.get(|builder| builder.get_mnemonic_with_passphrase().into())
    }

    pub fn get_indices_in_mnemonic_of_words_to_confirm(
        self: Arc<Self>,
    ) -> Vec<u8> {
        self.get(|builder| {
            builder
                .get_indices_in_mnemonic_of_words_to_confirm()
                .into_iter()
                .map(|i| i as u8)
                .collect::<Vec<_>>()
        })
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
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl DeviceFactorSourceBuilder {
    pub fn create_new_mnemonic_with_passphrase(self: Arc<Self>) -> Arc<Self> {
        self.set(|builder| builder.create_new_mnemonic_with_passphrase())
    }

    pub fn create_mnemonic_with_passphrase_from_words(
        self: Arc<Self>,
        words: Vec<BIP39Word>,
    ) -> Result<Arc<Self>> {
        self.set_from_result(|builder| {
            builder
                .create_mnemonic_with_passphrase_from_words(
                    words.clone().into_internal(),
                )
                .into_result()
        })
    }
}

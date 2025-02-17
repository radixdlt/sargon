use crate::prelude::*;
use sargon::DeviceMnemonicBuildResult as InternalDeviceMnemonicBuildResult;
use sargon::DeviceMnemonicBuilder as InternalDeviceMnemonicBuilder;

/// A builder of `MnemonicWithPassphrase` required for a new `DeviceFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct DeviceMnemonicBuilder {
    wrapped: Arc<InternalDeviceMnemonicBuilder>,
}

/// The result of the `DeviceMnemonicBuilder` representing the `build` function outcome,
/// before being able to use the underlying `MnemonicWithPassphrase`.
#[derive(PartialEq, uniffi::Enum)]
pub enum DeviceMnemonicBuildResult {
    /// The mnemonic words were confirmed
    Confirmed {
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    },
    /// The mnemonic words were unconfirmed
    Unconfirmed { words: HashMap<u8, String> },
}

impl From<InternalDeviceMnemonicBuildResult> for DeviceMnemonicBuildResult {
    fn from(value: InternalDeviceMnemonicBuildResult) -> Self {
        match value {
            InternalDeviceMnemonicBuildResult::Confirmed {
                mnemonic_with_passphrase,
            } => DeviceMnemonicBuildResult::Confirmed {
                mnemonic_with_passphrase: mnemonic_with_passphrase.into(),
            },
            InternalDeviceMnemonicBuildResult::Unconfirmed { words } => {
                DeviceMnemonicBuildResult::Unconfirmed {
                    words: words
                        .into_iter()
                        .map(|(k, v)| (k as u8, v))
                        .collect::<HashMap<_, _>>(),
                }
            }
        }
    }
}

#[uniffi::export]
impl DeviceMnemonicBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wrapped: Arc::new(InternalDeviceMnemonicBuilder::default()),
        })
    }
}

impl DeviceMnemonicBuilder {
    fn get<R>(
        &self,
        access: impl Fn(&InternalDeviceMnemonicBuilder) -> R,
    ) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalDeviceMnemonicBuilder>,
        ) -> &InternalDeviceMnemonicBuilder,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            _ = write(&builder.wrapped);
        })
    }

    fn set_from_result(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalDeviceMnemonicBuilder>,
        ) -> Result<&InternalDeviceMnemonicBuilder>,
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
impl DeviceMnemonicBuilder {
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

    pub fn factor_source_id(self: Arc<Self>) -> FactorSourceID {
        self.get(|builder| builder.factor_source_id().into())
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl DeviceMnemonicBuilder {
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

#[uniffi::export]
impl DeviceMnemonicBuilder {
    pub fn build(
        self: Arc<Self>,
        words_to_confirm: HashMap<u8, String>,
    ) -> DeviceMnemonicBuildResult {
        self.wrapped
            .build(
                &words_to_confirm
                    .into_iter()
                    .map(|(k, v)| (k as usize, v))
                    .collect::<HashMap<_, _>>(),
            )
            .into()
    }
}

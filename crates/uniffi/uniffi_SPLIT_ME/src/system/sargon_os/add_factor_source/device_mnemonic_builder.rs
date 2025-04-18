use crate::prelude::*;
use sargon::DeviceMnemonicValidationOutcome as InternalDeviceMnemonicValidationOutcome;
use sargon::DeviceMnemonicBuilder as InternalDeviceMnemonicBuilder;

/// A builder of `MnemonicWithPassphrase` required for a new `DeviceFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug, Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct DeviceMnemonicBuilder {
    wrapped: Arc<InternalDeviceMnemonicBuilder>,
}

/// The outcome of the `build` function from `DeviceMnemonicBuilder`.
#[derive(Debug, PartialEq, uniffi::Enum)]
pub enum DeviceMnemonicValidationOutcome {
    /// The mnemonic words were valid
    Valid,
    /// The mnemonic words were invalid
    Invalid {
        indices_in_mnemonic: Vec<u16>,
    },
}

impl From<InternalDeviceMnemonicValidationOutcome> for DeviceMnemonicValidationOutcome {
    fn from(value: InternalDeviceMnemonicValidationOutcome) -> Self {
        match value {
            InternalDeviceMnemonicValidationOutcome::Valid => DeviceMnemonicValidationOutcome::Valid,
            InternalDeviceMnemonicValidationOutcome::Invalid {
                indices_in_mnemonic,
            } => DeviceMnemonicValidationOutcome::Invalid {
                indices_in_mnemonic: indices_in_mnemonic
                    .into_iter()
                    .map(|i| i as u16)
                    .sorted()
                    .collect::<Vec<_>>(),
            },
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
    pub fn get_mnemonic_with_passphrase(self: Arc<Self>) -> MnemonicWithPassphrase {
        self.get(|builder| builder.get_mnemonic_with_passphrase().into())
    }

    /// Returns the words of the mnemonic with passphrase.
    pub fn get_words(self: Arc<Self>) -> Vec<BIP39Word> {
        self.get(|builder| {
            builder.get_words().into_iter().map(Into::into).collect()
        })
    }

    /// Get a set of words indices within `MnemonicWithPassphrase` to be confirmed.
    /// Always includes the last mnemonic word index.
    pub fn get_indices_in_mnemonic_of_words_to_confirm(
        self: Arc<Self>,
    ) -> Vec<u16> {
        self.get(|builder| {
            builder
                .get_indices_in_mnemonic_of_words_to_confirm()
                .into_iter()
                .map(|i| i as u16)
                .collect::<Vec<_>>()
        })
    }

    /// Returns the `FactorSourceID` from the mnemonic with passphrase
    /// Panics if the mnemonic with passphrase wasn't yet created
    pub fn get_factor_source_id(self: Arc<Self>) -> FactorSourceID {
        self.get(|builder| builder.get_factor_source_id().into())
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl DeviceMnemonicBuilder {
    /// Generates a new mnemonic
    pub fn generate_new_mnemonic(self: Arc<Self>) -> Arc<Self> {
        self.set(|builder| builder.generate_new_mnemonic())
    }

    /// Creates a new mnemonic from given `words`
    pub fn create_mnemonic_from_words(
        self: Arc<Self>,
        words: Vec<String>,
    ) -> Result<Arc<Self>> {
        self.set_from_result(|builder| {
            builder
                .create_mnemonic_from_words(words.clone())
                .into_result()
        })
    }
}

#[uniffi::export]
impl DeviceMnemonicBuilder {
    /// Verifies if the `words_to_confirm` contains the expected number of words.
    /// Verifies if the `words_to_confirm` are correct within the previously created `MnemonicWithPassphrase`.
    /// Returns unconfirmed words if not all of the words were confirmed or the `MnemonicWithPassphrase`.
    pub fn validate_words(
        self: Arc<Self>,
        words_to_confirm: HashMap<u16, String>,
    ) -> DeviceMnemonicValidationOutcome {
        self.wrapped
            .validate_words(
                &words_to_confirm
                    .into_iter()
                    .map(|(k, v)| (k as usize, v))
                    .collect::<HashMap<_, _>>(),
            )
            .into()
    }
}
use crate::prelude::*;
use sargon::MnemonicBuilder as InternalMnemonicBuilder;
use sargon::MnemonicValidationOutcome as InternalMnemonicValidationOutcome;

/// A builder of `MnemonicWithPassphrase` required for a new `DeviceFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug, Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct MnemonicBuilder {
    wrapped: Arc<InternalMnemonicBuilder>,
}

/// The outcome of the `build` function from `MnemonicBuilder`.
#[derive(Debug, PartialEq, uniffi::Enum)]
pub enum MnemonicValidationOutcome {
    /// The mnemonic words were valid
    Valid,
    /// The mnemonic words were invalid
    Invalid { indices_in_mnemonic: Vec<u16> },
}

impl From<InternalMnemonicValidationOutcome> for MnemonicValidationOutcome {
    fn from(value: InternalMnemonicValidationOutcome) -> Self {
        match value {
            InternalMnemonicValidationOutcome::Valid => {
                MnemonicValidationOutcome::Valid
            }
            InternalMnemonicValidationOutcome::Invalid {
                indices_in_mnemonic,
            } => MnemonicValidationOutcome::Invalid {
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
impl MnemonicBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wrapped: Arc::new(InternalMnemonicBuilder::default()),
        })
    }
}

impl MnemonicBuilder {
    fn get<R>(&self, access: impl Fn(&InternalMnemonicBuilder) -> R) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        write: impl Fn(&Arc<InternalMnemonicBuilder>) -> &InternalMnemonicBuilder,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            _ = write(&builder.wrapped);
        })
    }

    fn set_from_result(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalMnemonicBuilder>,
        ) -> Result<&InternalMnemonicBuilder>,
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
impl MnemonicBuilder {
    pub fn get_mnemonic_with_passphrase(
        self: Arc<Self>,
    ) -> MnemonicWithPassphrase {
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
    pub fn get_factor_source_id(
        self: Arc<Self>,
        kind: FactorSourceKind,
    ) -> FactorSourceID {
        self.get(|builder| {
            builder.get_factor_source_id(kind.into_internal()).into()
        })
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl MnemonicBuilder {
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

    /// Updates the passphrase associated with the managed mnemonic.
    pub fn set_passphrase(
        self: Arc<Self>,
        passphrase: BIP39Passphrase,
    ) -> Arc<Self> {
        self.set(|builder| builder.set_passphrase(passphrase.into_internal()))
    }
}

#[uniffi::export]
impl MnemonicBuilder {
    /// Verifies if the `words_to_confirm` contains the expected number of words.
    /// Verifies if the `words_to_confirm` are correct within the previously created `MnemonicWithPassphrase`.
    /// Returns unconfirmed words if not all of the words were confirmed or the `MnemonicWithPassphrase`.
    pub fn validate_words(
        self: Arc<Self>,
        words_to_confirm: HashMap<u16, String>,
    ) -> MnemonicValidationOutcome {
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

use crate::prelude::*;
use sargon::DeviceMnemonicBuildResult as InternalDeviceMnemonicBuildResult;
use sargon::DeviceMnemonicBuilder as InternalDeviceMnemonicBuilder;

/// A builder of `MnemonicWithPassphrase` required for a new `DeviceFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug, Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct DeviceMnemonicBuilder {
    wrapped: Arc<InternalDeviceMnemonicBuilder>,
}

/// The result of the `DeviceMnemonicBuilder` representing the `build` function outcome,
/// before being able to use the underlying `MnemonicWithPassphrase`.
#[derive(Debug, PartialEq, uniffi::Enum)]
pub enum DeviceMnemonicBuildResult {
    /// The mnemonic words were confirmed
    Confirmed {
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    },
    /// The number of words to confirm was incorrect
    ConfirmationWordCountMismatch,
    /// The mnemonic words were unconfirmed
    Unconfirmed { indices_in_mnemonic: Vec<u8> },
}

impl From<InternalDeviceMnemonicBuildResult> for DeviceMnemonicBuildResult {
    fn from(value: InternalDeviceMnemonicBuildResult) -> Self {
        match value {
            InternalDeviceMnemonicBuildResult::Confirmed {
                mnemonic_with_passphrase,
            } => DeviceMnemonicBuildResult::Confirmed {
                mnemonic_with_passphrase: mnemonic_with_passphrase.into(),
            },
            InternalDeviceMnemonicBuildResult::ConfirmationWordCountMismatch => DeviceMnemonicBuildResult::ConfirmationWordCountMismatch,
            InternalDeviceMnemonicBuildResult::Unconfirmed {
                indices_in_mnemonic,
            } => DeviceMnemonicBuildResult::Unconfirmed {
                indices_in_mnemonic: indices_in_mnemonic
                    .into_iter()
                    .map(|i| i as u8)
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
    /// Get a set of words indices within `MnemonicWithPassphrase` to be confirmed.
    /// Always includes the last mnemonic word index.
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

    /// Returns the `FactorSourceID` from the mnemonic with passphrase
    /// Panics if the mnemonic with passphrase wasn't yet created
    pub fn factor_source_id(self: Arc<Self>) -> FactorSourceID {
        self.get(|builder| builder.factor_source_id().into())
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl DeviceMnemonicBuilder {
    /// Generates a new mnemonic
    pub fn create_new_mnemonic_with_passphrase(self: Arc<Self>) -> Arc<Self> {
        self.set(|builder| builder.create_new_mnemonic_with_passphrase())
    }

    /// Creates a new mnemonic from given `words`
    pub fn create_mnemonic_with_passphrase_from_words(
        self: Arc<Self>,
        words: Vec<String>,
    ) -> Result<Arc<Self>> {
        self.set_from_result(|builder| {
            builder
                .create_mnemonic_with_passphrase_from_words(words.clone())
                .into_result()
        })
    }
}

#[uniffi::export]
impl DeviceMnemonicBuilder {
    /// Verifies if the `words_to_confirm` contains the expected number of words.
    /// Verifies if the `words_to_confirm` are correct within the previously created `MnemonicWithPassphrase`.
    /// Returns unconfirmed words if not all of the words were confirmed or the `MnemonicWithPassphrase`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceMnemonicBuilder;

    #[test]
    fn build() {
        let mnemonic_words = sargon::Mnemonic::sample_device()
            .words
            .iter()
            .map(|w| w.word.clone())
            .collect::<Vec<_>>();
        let sut = SUT::new()
            .create_new_mnemonic_with_passphrase()
            .create_mnemonic_with_passphrase_from_words(mnemonic_words.clone()) // Override the previously created mnemonic and create a new one from words
            .unwrap();

        let indices = sut.clone().get_indices_in_mnemonic_of_words_to_confirm();

        let fsid = sut.clone().factor_source_id();

        pretty_assertions::assert_eq!(fsid, FactorSourceID::sample_device());

        let r0 = sut.clone().build(
            vec![(0, "device".to_owned())]
                .into_iter()
                .collect::<HashMap<_, _>>(),
        ); // Input not enough words

        pretty_assertions::assert_eq!(
            r0,
            DeviceMnemonicBuildResult::ConfirmationWordCountMismatch
        );

        let r1 = sut.clone().build(
            vec![
                (0, "device".to_owned()),
                (5, "word".to_owned()),
                (11, "sign".to_owned()),
                (17, "about".to_owned()),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        ); // Input the incorrect words

        pretty_assertions::assert_eq!(
            r1,
            DeviceMnemonicBuildResult::Unconfirmed {
                indices_in_mnemonic: vec![5, 11, 17]
            }
        );

        let r2 = sut.build(
            indices
                .into_iter()
                .map(|i| (i, mnemonic_words[i as usize].clone()))
                .collect::<HashMap<_, _>>(),
        ); // Confirm the words based on previously generated indices

        pretty_assertions::assert_eq!(
            r2,
            DeviceMnemonicBuildResult::Confirmed {
                mnemonic_with_passphrase:
                    sargon::MnemonicWithPassphrase::sample_device().into(),
            }
        );
    }
}

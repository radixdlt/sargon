use crate::prelude::*;
use sargon::OffDeviceMnemonicBuilder as InternalOffDeviceMnemonicBuilder;

/// A builder of `MnemonicWithPassphrase` required for a new `OffDeviceMnemonicFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug, Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct OffDeviceMnemonicBuilder {
    wrapped: Arc<InternalOffDeviceMnemonicBuilder>,
}

#[uniffi::export]
impl OffDeviceMnemonicBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wrapped: Arc::new(InternalOffDeviceMnemonicBuilder::default()),
        })
    }
}

impl OffDeviceMnemonicBuilder {
    fn get<R>(
        &self,
        access: impl Fn(&InternalOffDeviceMnemonicBuilder) -> R,
    ) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalOffDeviceMnemonicBuilder>,
        ) -> &InternalOffDeviceMnemonicBuilder,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            _ = write(&builder.wrapped);
        })
    }

    fn set_from_result(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalOffDeviceMnemonicBuilder>,
        ) -> Result<&InternalOffDeviceMnemonicBuilder>,
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
impl OffDeviceMnemonicBuilder {
    /// Returns the words of the mnemonic.
    pub fn get_words(self: Arc<Self>) -> Vec<BIP39Word> {
        self.get(|builder| {
            builder.get_words().into_iter().map(Into::into).collect()
        })
    }

    /// Returns the word count of the mnemonic.
    pub fn get_word_count(self: Arc<Self>) -> BIP39WordCount {
        self.get(|builder| builder.get_word_count().into())
    }

    /// Returns the `FactorSourceID` from the mnemonic
    pub fn get_factor_source_id(self: Arc<Self>) -> FactorSourceID {
        self.get(|builder| builder.get_factor_source_id().into())
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl OffDeviceMnemonicBuilder {
    /// Generates a new mnemonic with specified `word_count` and sets it as the `mnemonic_with_passphrase`.
    pub fn generate_new_mnemonic(
        self: Arc<Self>,
        word_count: BIP39WordCount,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.generate_new_mnemonic(word_count.into_internal())
        })
    }

    /// Creates a new mnemonic from given `words`
    pub fn create_mnemonic(
        self: Arc<Self>,
        words: Vec<String>,
    ) -> Result<Arc<Self>> {
        self.set_from_result(|builder| {
            builder.create_mnemonic(words.clone()).into_result()
        })
    }
}

#[uniffi::export]
impl OffDeviceMnemonicBuilder {
    /// This doesn't actually build anything, it just returns the `MnemonicWithPassphrase`.
    /// Kept the name for consistency with other builders.
    pub fn build(self: Arc<Self>) -> MnemonicWithPassphrase {
        self.wrapped.build().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OffDeviceMnemonicBuilder;

    #[test]
    fn build() {
        let mnemonic = sargon::Mnemonic::sample_off_device();
        let mnemonic_words = mnemonic
            .clone()
            .words
            .iter()
            .map(|w| w.word.clone())
            .collect::<Vec<_>>();
        let sut = SUT::new()
            .generate_new_mnemonic(BIP39WordCount::TwentyFour)
            .create_mnemonic(mnemonic_words.clone()) // Override the previously created mnemonic and create a new one from words
            .unwrap();

        pretty_assertions::assert_eq!(
            sut.clone().get_words(),
            mnemonic
                .clone()
                .words
                .into_iter()
                .map(|w| w.clone().into())
                .collect::<Vec<_>>()
        );
        pretty_assertions::assert_eq!(
            sut.clone().build(),
            sargon::MnemonicWithPassphrase::new(mnemonic).into()
        );

        pretty_assertions::assert_eq!(
            sut.clone().get_factor_source_id(),
            FactorSourceID::sample_off_device()
        );
    }
}

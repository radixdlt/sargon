use crate::prelude::*;
use sargon::PasswordMnemonicBuilder as InternalPasswordMnemonicBuilder;

/// A builder of `MnemonicWithPassphrase` required for a new `PasswordFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug, Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct PasswordMnemonicBuilder {
    wrapped: Arc<InternalPasswordMnemonicBuilder>,
}

#[uniffi::export]
impl PasswordMnemonicBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wrapped: Arc::new(InternalPasswordMnemonicBuilder::default()),
        })
    }
}

impl PasswordMnemonicBuilder {
    fn get<R>(
        &self,
        access: impl Fn(&InternalPasswordMnemonicBuilder) -> R,
    ) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set_from_result(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<InternalPasswordMnemonicBuilder>,
        ) -> Result<&InternalPasswordMnemonicBuilder>,
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
impl PasswordMnemonicBuilder {
    /// Returns the `FactorSourceID` from the mnemonic
    pub fn get_factor_source_id(self: Arc<Self>) -> FactorSourceID {
        self.get(|builder| builder.get_factor_source_id().into())
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl PasswordMnemonicBuilder {
    /// Creates a new mnemonic from given `password`
    /// `password` and `confirm_password` should be the same.
    pub fn create_mnemonic(
        self: Arc<Self>,
        password: String,
        confirm_password: String,
    ) -> Result<Arc<Self>> {
        self.set_from_result(|builder| {
            builder
                .create_mnemonic(password.clone(), confirm_password.clone())
                .into_result()
        })
    }
}

#[uniffi::export]
impl PasswordMnemonicBuilder {
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
    type SUT = PasswordMnemonicBuilder;

    #[test]
    fn build() {
        let sut = SUT::new()
            .create_mnemonic(
                "securepassword123".to_owned(),
                "securepassword123".to_owned(),
            )
            .unwrap();
        let internal_mnemonic = sargon::MnemonicWithPassphrase::new(sargon::Mnemonic::from_phrase(
            "slice divert oppose salon poverty chalk educate twelve essence celery chuckle park sail clutch clutch teach video eyebrow skill renew random attend guide quarter"
        ).unwrap());

        pretty_assertions::assert_eq!(
            sut.clone().build(),
            internal_mnemonic.clone().into()
        );

        pretty_assertions::assert_eq!(
            sut.clone().get_factor_source_id(),
            sargon::FactorSourceID::Hash {
                value: sargon::FactorSourceIDFromHash::new_for_password(
                    &internal_mnemonic
                )
            }
            .into()
        );
    }
}

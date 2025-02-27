use encryption::PasswordBasedKeyDerivationScheme;

use crate::prelude::*;

/// A builder of `MnemonicWithPassphrase` required for a new `PasswordFactorSource` creation.
/// Exposes functions to be called by hosts to be able to use the resulting `MnemonicWithPassphrase`.
#[derive(Debug)]
pub struct PasswordMnemonicBuilder {
    mnemonic_with_passphrase: RwLock<Option<MnemonicWithPassphrase>>,
}

impl Default for PasswordMnemonicBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for PasswordMnemonicBuilder {
    fn eq(&self, other: &Self) -> bool {
        let mnemonic_with_passphrase =
            self.mnemonic_with_passphrase.read().unwrap();
        let other_mnemonic_with_passphrase =
            other.mnemonic_with_passphrase.read().unwrap();
        *mnemonic_with_passphrase == *other_mnemonic_with_passphrase
    }
}

impl Eq for PasswordMnemonicBuilder {}

impl std::hash::Hash for PasswordMnemonicBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mnemonic_with_passphrase =
            self.mnemonic_with_passphrase.read().unwrap();
        mnemonic_with_passphrase.hash(state);
    }
}

impl HasSampleValues for PasswordMnemonicBuilder {
    fn sample() -> Self {
        Self::with_mnemonic_with_passphrase(MnemonicWithPassphrase::sample())
    }

    fn sample_other() -> Self {
        Self::with_mnemonic_with_passphrase(
            MnemonicWithPassphrase::sample_other(),
        )
    }
}

impl PasswordMnemonicBuilder {
    const MINIMUM_NUMBER_OF_CHARACTERS: usize = 16;

    fn new() -> Self {
        Self {
            mnemonic_with_passphrase: RwLock::new(None),
        }
    }

    fn with_mnemonic_with_passphrase(
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    ) -> Self {
        Self {
            mnemonic_with_passphrase: RwLock::new(Some(
                mnemonic_with_passphrase,
            )),
        }
    }
}

// ====================
// ==== GET / READ ====
// ====================
impl PasswordMnemonicBuilder {
    /// Returns the `mnemonic_with_passphrase` if it was previously created or panics.
    fn get_mnemonic_with_passphrase(&self) -> MnemonicWithPassphrase {
        self.mnemonic_with_passphrase
            .read()
            .unwrap()
            .clone()
            .expect("Mnemonic should be created first")
    }
}

// ====================
// ===== MUTATION =====
// ====================
impl PasswordMnemonicBuilder {
    /// Creates a new mnemonic from given `password`.
    /// `password` and `confirm_password` should be the same.
    pub fn create_mnemonic(
        &self,
        password: impl AsRef<str>,
        confirm_password: impl AsRef<str>,
    ) -> Result<&Self> {
        if password.as_ref().len() < Self::MINIMUM_NUMBER_OF_CHARACTERS {
            return Err(CommonError::PasswordTooShort);
        }

        if password.as_ref() != confirm_password.as_ref() {
            return Err(CommonError::PasswordsDoNotMatch);
        }

        let entropy_bytes =
            PasswordBasedKeyDerivationScheme::version1().kdf(password.as_ref());
        let mnemonic = Mnemonic::from_32bytes_entropy(entropy_bytes);
        self.set_mnemonic_with_passphrase(mnemonic);
        Ok(self)
    }

    fn set_mnemonic_with_passphrase(&self, mnemonic: Mnemonic) {
        *self.mnemonic_with_passphrase.write().unwrap() =
            Some(MnemonicWithPassphrase::new(mnemonic));
    }
}

impl PasswordMnemonicBuilder {
    /// Returns the `FactorSourceID` from the mnemonic
    /// Panics if the mnemonic wasn't yet created
    pub fn get_factor_source_id(&self) -> FactorSourceID {
        FactorSourceID::from(FactorSourceIDFromHash::new_for_password(
            &self.get_mnemonic_with_passphrase(),
        ))
    }

    /// This doesn't actually build anything, it just returns the `MnemonicWithPassphrase`.
    /// Kept the name for consistency with other builders.
    pub fn build(&self) -> MnemonicWithPassphrase {
        self.get_mnemonic_with_passphrase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PasswordMnemonicBuilder;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    #[allow(clippy::mutable_key_type)]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample(),
                SUT::sample(),
                SUT::sample_other(),
                SUT::sample_other(),
            ])
            .len(),
            2
        )
    }

    #[test]
    fn create_mnemonic_fails_when_password_is_too_short() {
        let sut = SUT::default();

        let result = sut.create_mnemonic("password", "password");

        pretty_assertions::assert_eq!(
            result,
            Err(CommonError::PasswordTooShort)
        );
    }

    #[test]
    fn create_mnemonic_fails_when_passwords_do_not_match() {
        let sut = SUT::default();

        let result = sut.create_mnemonic("securepassword123", "a");

        pretty_assertions::assert_eq!(
            result,
            Err(CommonError::PasswordsDoNotMatch)
        );
    }

    #[test]
    fn create_mnemonic_success() {
        let sut = SUT::default();

        let result =
            sut.create_mnemonic("securepassword123", "securepassword123");

        pretty_assertions::assert_eq!(
            result.unwrap().get_mnemonic_with_passphrase().mnemonic.phrase(),
            "slice divert oppose salon poverty chalk educate twelve essence celery chuckle park sail clutch clutch teach video eyebrow skill renew random attend guide quarter".to_owned()
        )
    }

    #[test]
    #[should_panic(expected = "Mnemonic should be created first")]
    fn get_factor_source_id_panics_if_mnemonic_not_created() {
        let sut = SUT::default();
        let _ = sut.get_factor_source_id();
    }

    #[test]
    fn get_factor_source_id() {
        let sut = SUT::sample();
        let fsid = sut.get_factor_source_id();
        pretty_assertions::assert_eq!(
            fsid,
            FactorSourceID::from(FactorSourceIDFromHash::new_for_password(
                &MnemonicWithPassphrase::sample()
            ))
        );
    }

    #[test]
    fn build() {
        let sut = SUT::sample();

        let result = sut.build();

        pretty_assertions::assert_eq!(result, MnemonicWithPassphrase::sample());
    }
}

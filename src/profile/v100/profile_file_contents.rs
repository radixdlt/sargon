use crate::prelude::*;

/// Result of analyzing a file (bytes), containing either a Profile
/// which we were able to successfully JSON deserialize from the bytes,
/// or EncryptedProfile for which wallets will continue prompting the
/// user for an encryption password and then call JSON deserialize
/// of `EncryptedProfileSnapshot` using [`Profile::new_from_encryption_bytes`](Profile::new_from_encryption_bytes)
/// or if we failed to parse as Profile and `EncryptedProfileSnapshot`
/// then `NotProfile` is used, indicating that the bytes is not at all
/// a Profile.
#[derive(Debug, PartialEq, Eq, uniffi::Enum)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum ProfileFileContents {
    /// The JSON deserialized Profile from some bytes.
    Plaintext { reference: Arc<RefProfile> },

    /// We successfully JSON deserialized the bytes into
    /// `EncryptedProfileSnapshot`, the wallets should proceed
    /// with asking the user for the decryption password.
    Encrypted,

    /// The bytes is neither a valid `Profile` nor `EncryptedProfile`,
    /// it is either a corrupt file or it is not at all a Profile file,
    /// contrary to the users beliefs (or the user accidentally selected
    /// a random file...)
    NotProfile,
}

impl std::hash::Hash for ProfileFileContents {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self {
            Self::Plaintext { reference } => {
                state.write_u8(1);
                (*reference).hash(state);
            }

            Self::Encrypted => {
                state.write_u8(2);
            }

            Self::NotProfile => {
                state.write_u8(3);
            }
        }
    }
}

impl HasSampleValues for ProfileFileContents {
    fn sample() -> Self {
        Self::Plaintext {
            reference: RefProfile::new(Profile::sample()),
        }
    }

    fn sample_other() -> Self {
        Self::Encrypted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileFileContents;

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
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample(),
                SUT::sample_other(),
                // duplicates should get removed
                SUT::sample(),
                SUT::sample_other()
            ])
            .len(),
            2
        );
    }
}

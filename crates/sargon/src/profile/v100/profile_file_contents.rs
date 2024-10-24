use crate::prelude::*;

/// Result of analyzing a file (bytes), containing either a Profile
/// which we were able to successfully JSON deserialize from the bytes,
/// or EncryptedProfile for which wallets will continue prompting the
/// user for an encryption password and then call JSON deserialize
/// of `EncryptedProfileSnapshot` using [`Profile::new_from_encryption_bytes`](Profile::new_from_encryption_bytes)
/// or if we failed to parse as Profile and `EncryptedProfileSnapshot`
/// then `NotProfile` is used, indicating that the bytes is not at all
/// a Profile.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum ProfileFileContents {
    /// The JSON deserialized Profile from some bytes.
    PlaintextProfile(Profile),

    /// We successfully JSON deserialized the bytes into
    /// `EncryptedProfileSnapshot`, the wallets should proceed
    /// with asking the user for the decryption password.
    EncryptedProfile,

    /// The bytes is neither a valid `Profile` nor `EncryptedProfile`,
    /// it is either a corrupt file or it is not at all a Profile file,
    /// contrary to the users beliefs (or the user accidentally selected
    /// a random file...)
    NotProfile,
}

impl HasSampleValues for ProfileFileContents {
    fn sample() -> Self {
        Self::PlaintextProfile(Profile::sample())
    }

    fn sample_other() -> Self {
        Self::EncryptedProfile
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
                SUT::NotProfile,
                // duplicates should get removed
                SUT::sample(),
                SUT::sample_other(),
                SUT::NotProfile,
            ])
            .len(),
            3
        );
    }
}

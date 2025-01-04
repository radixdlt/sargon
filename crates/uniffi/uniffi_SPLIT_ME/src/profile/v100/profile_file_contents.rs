use crate::prelude::*;
use sargon::ProfileFileContents as InternalProfileFileContents;

/// Result of analyzing a file (bytes), containing either a Profile
/// which we were able to successfully JSON deserialize from the bytes,
/// or EncryptedProfile for which wallets will continue prompting the
/// user for an encryption password and then call JSON deserialize
/// of `EncryptedProfileSnapshot` using [`Profile::new_from_encryption_bytes`](Profile::new_from_encryption_bytes)
/// or if we failed to parse as Profile and `EncryptedProfileSnapshot`
/// then `NotProfile` is used, indicating that the bytes is not at all
/// a Profile.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
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

#[uniffi::export]
pub(crate) fn new_profile_file_contents_sample() -> ProfileFileContents {
    InternalProfileFileContents::sample().into()
}

#[uniffi::export]
pub(crate) fn new_profile_file_contents_sample_other() -> ProfileFileContents {
    InternalProfileFileContents::sample_other().into()
}

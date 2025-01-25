use encryption::{
    EncryptionKey, EncryptionScheme, PasswordBasedKeyDerivationScheme,
    VersionedEncryption, VersionedPasswordBasedKeyDerivation,
};
use prelude::fixture_profiles;

use crate::prelude::*;

/// An encryption of a `ProfileSnapshot` with crypto metadata about how it was encrypted, which can
/// be used to decrypt it, given a user provided password.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}: {}", version, key_derivation_scheme)]
pub struct EncryptedProfileSnapshot {
    /// JSON format version of this struct
    pub version: ProfileEncryptionVersion,

    /// Encrypted JSON encoding of a `ProfileSnapshot`
    #[serde(rename = "encryptedSnapshot")]
    pub encrypted_snapshot: BagOfBytes,

    /// The KDF algorithm which was used to derive the encryption key from the user provided password.
    #[serde(rename = "keyDerivationScheme")]
    pub key_derivation_scheme: PasswordBasedKeyDerivationScheme,

    /// The encryption algorithm which was used to produce `encryptedSnapshot` with the encryption key
    /// derived using the `keyDerivationScheme`.
    #[serde(rename = "encryptionScheme")]
    pub encryption_scheme: EncryptionScheme,
}

impl EncryptedProfileSnapshot {
    pub fn decrypt(&self, password: impl AsRef<str>) -> Result<Profile> {
        // decrypt Profile JSON bytes
        let decrypted = self.decrypt_to_bytes(password)?;

        // JSON decode bytes into Profile
        decrypted.deserialize()
    }

    pub fn decrypt_to_bytes(
        &self,
        password: impl AsRef<str>,
    ) -> Result<Vec<u8>> {
        // Derive encryption key based on password
        let mut decryption_key =
            EncryptionKey::from(self.key_derivation_scheme.kdf(password));

        // decrypt Profile JSON bytes
        self.encryption_scheme
            .decrypt(self.encrypted_snapshot.to_vec(), &mut decryption_key)
    }

    pub fn encrypting(
        profile: &Profile,
        password: impl AsRef<str>,
        kdf_scheme: impl Into<Option<PasswordBasedKeyDerivationScheme>>,
        encryption_scheme: impl Into<Option<EncryptionScheme>>,
    ) -> Self {
        let key_derivation_scheme = kdf_scheme.into().unwrap_or_default();
        let encryption_scheme = encryption_scheme.into().unwrap_or_default();

        // JSON encode profile
        let json = profile.serialize_to_bytes().unwrap();

        // derive symmetric encryption key
        let mut encryption_key =
            EncryptionKey::from(key_derivation_scheme.kdf(password));

        // encrypt profile with encryption key
        let encrypted_payload =
            encryption_scheme.encrypt(&json, &mut encryption_key);

        Self {
            version: ProfileEncryptionVersion::default(),
            encrypted_snapshot: BagOfBytes::from(encrypted_payload),
            key_derivation_scheme,
            encryption_scheme,
        }
    }
}

impl HasSampleValues for EncryptedProfileSnapshot {
    /// Password is: `"babylon"` - encryption of SAME profile as `Self::sample_other()`
    fn sample() -> Self {
        let json_str =
            fixture_profiles!("profile_encrypted_by_password_of_babylon");
        serde_json::from_str::<EncryptedProfileSnapshot>(json_str).unwrap()
    }

    /// Password is: `""` (empty) - encryption of SAME profile as `Self::sample()`
    fn sample_other() -> Self {
        let json_str = fixture_profiles!("profile_encrypted_by_password_empty");
        serde_json::from_str::<EncryptedProfileSnapshot>(json_str).unwrap()
    }
}

#[derive(
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(transparent)]
pub struct ProfileEncryptionVersion(u32);

impl Default for ProfileEncryptionVersion {
    fn default() -> Self {
        Self(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EncryptedProfileSnapshot;

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
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_json_roundtrip(&sut);
    }

    #[test]
    fn decrypt_sample() {
        let sut = SUT::sample();
        let decrypted = sut.decrypt("babylon").unwrap();
        assert_eq!(
            decrypted.header.id,
            ProfileID::from_str("e5e4477b-e47b-4b64-bbc8-f8f40e8beb74")
                .unwrap()
        );
    }

    #[test]
    fn decrypt_sample_other() {
        let sut = SUT::sample_other();
        let decrypted = sut.decrypt("").unwrap();
        assert_eq!(
            decrypted.header.id,
            ProfileID::from_str("e5e4477b-e47b-4b64-bbc8-f8f40e8beb74")
                .unwrap()
        );
    }

    #[test]
    fn decrypt_samples() {
        let decrypted_sample = SUT::sample().decrypt("babylon").unwrap();
        let decrypted_sample_other = SUT::sample_other().decrypt("").unwrap();
        assert_eq!(decrypted_sample, decrypted_sample_other);
    }

    #[test]
    fn encryption_roundtrip() {
        let test = |profile: Profile, password: &str| {
            let encrypted = SUT::encrypting(&profile, password, None, None);
            let decrypted = encrypted.decrypt(password).unwrap();
            assert_eq!(decrypted, profile);
        };

        let password = "so secure";
        test(Profile::sample(), password);
        test(Profile::sample_other(), password);

        let password = "even more secure";
        test(Profile::sample(), password);
        test(Profile::sample_other(), password);
    }
}

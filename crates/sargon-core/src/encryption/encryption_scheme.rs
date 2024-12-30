use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
pub enum EncryptionScheme {
    /// AES GCM 256 encryption
    Version1(AesGcm256),
}

impl HasSampleValues for EncryptionScheme {
    fn sample() -> Self {
        Self::version1()
    }

    fn sample_other() -> Self {
        Self::version1()
    }
}

impl std::fmt::Display for EncryptionScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EncryptionScheme: {} ({})",
            self.version(),
            self.description()
        )
    }
}

impl Serialize for EncryptionScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EncryptionScheme", 2)?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("version", &self.version())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for EncryptionScheme {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            version: EncryptionSchemeVersion,
        }
        Wrapper::deserialize(deserializer)
            .and_then(|w| Self::try_from(w.version).map_err(de::Error::custom))
    }
}

impl EncryptionScheme {
    pub fn version1() -> Self {
        Self::Version1(AesGcm256::default())
    }
}

impl Default for EncryptionScheme {
    fn default() -> Self {
        Self::version1()
    }
}

impl VersionedEncryption for EncryptionScheme {
    /// Encrypts `plaintext` using `encryption_key` using
    /// the `self` `EncryptionScheme`, returning the cipher text as Vec<u8>.
    fn encrypt(
        &self,
        plaintext: impl AsRef<[u8]>,
        encryption_key: &mut EncryptionKey,
    ) -> Vec<u8> {
        match self {
            EncryptionScheme::Version1(scheme) => {
                scheme.encrypt(plaintext, encryption_key)
            }
        }
    }

    /// Tries to decrypt the `cipher_text` using the `decryption_key` according
    /// to the `self` `EncryptionScheme`, returning the plaintext if operation
    /// was successful.
    fn decrypt(
        &self,
        cipher_text: impl AsRef<[u8]>,
        decryption_key: &mut EncryptionKey,
    ) -> Result<Vec<u8>> {
        match self {
            EncryptionScheme::Version1(scheme) => {
                scheme.decrypt(cipher_text, decryption_key)
            }
        }
    }
}

impl TryFrom<EncryptionSchemeVersion> for EncryptionScheme {
    type Error = CommonError;
    fn try_from(value: EncryptionSchemeVersion) -> Result<Self> {
        match value {
            EncryptionSchemeVersion::Version1 => Ok(Self::version1()),
        }
    }
}
impl VersionOfAlgorithm for EncryptionScheme {
    type Version = EncryptionSchemeVersion;

    fn version(&self) -> Self::Version {
        match self {
            Self::Version1(scheme) => scheme.version(),
        }
    }

    fn description(&self) -> String {
        match self {
            EncryptionScheme::Version1(scheme) => scheme.description(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EncryptionScheme;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::default()),
            "EncryptionScheme: Version1 (AESGCM-256)"
        );
    }

    #[test]
    fn json_() {
        let model = SUT::default();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
			"version": 1,
			"description": "AESGCM-256"
		}
        "#,
        );
    }

    #[test]
    fn encryption_roundtrip() {
        let sut = SUT::default();
        let mut encryption_key = EncryptionKey::generate();
        let mut decryption_key = encryption_key;
        let msg = "Hello Radix";
        let msg_bytes: Vec<u8> = msg.bytes().collect();

        let encrypted = sut.encrypt(&msg_bytes, &mut encryption_key);
        assert_eq!(encryption_key.0, Exactly32Bytes::from(&[0; 32])); // assert zeroed out

        let decrypted_bytes =
            sut.decrypt(encrypted, &mut decryption_key).unwrap();
        assert_eq!(decryption_key.0, Exactly32Bytes::from(&[0; 32])); // assert zeroed out

        let decrypted = String::from_utf8(decrypted_bytes).unwrap();
        assert_eq!(msg, decrypted);
    }

    #[test]
    fn decrypt_known() {
        let sut = SUT::default();
        let test = |encrypted_hex: &str,
                    key_hex: &str,
                    expected_plaintext: &str| {
            let mut decryption_key = EncryptionKey::from_str(key_hex).unwrap();
            let encrypted = hex_decode(encrypted_hex).unwrap();
            let decrypted =
                sut.decrypt(encrypted, &mut decryption_key).unwrap();
            assert_eq!(hex::encode(decrypted), expected_plaintext);
        };

        test(
            "4c2266de48fd17a4bb52d5883751d054258755ce004154ea204a73a4c35e",
            "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            "abba",
        );
    }

    #[test]
    fn decrypt_invalid_sealed_box_is_err() {
        let sut = SUT::default();
        assert_eq!(
            sut.decrypt(Vec::new(), &mut EncryptionKey::sample()),
            Err(CommonError::InvalidAESBytesTooShort {
                expected_at_least: AesGcmSealedBox::LOWER_BOUND_LEN as u64,
                found: 0
            })
        );
    }
}

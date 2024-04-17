use crate::prelude::*;

#[derive(
    Clone, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug,
)]
pub enum EncryptionScheme {
    Version1(EncryptionSchemeVersion1),
}

#[cfg(not(tarpaulin_include))] // false negative
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

#[cfg(not(tarpaulin_include))] // false negative
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
        Self::Version1(EncryptionSchemeVersion1::default())
    }
}

impl Default for EncryptionScheme {
    fn default() -> Self {
        Self::version1()
    }
}

impl VersionedEncryption for EncryptionScheme {
    fn encrypt(
        &self,
        data: Vec<u8>,
        encryption_key: &Exactly32Bytes,
    ) -> Vec<u8> {
        match self {
            EncryptionScheme::Version1(scheme) => {
                scheme.encrypt(data, encryption_key)
            }
        }
    }

    fn decrypt(
        &self,
        data: Vec<u8>,
        decryption_key: &Exactly32Bytes,
    ) -> Result<Vec<u8>> {
        match self {
            EncryptionScheme::Version1(scheme) => {
                scheme.decrypt(data, decryption_key)
            }
        }
    }
}

impl TryFrom<EncryptionSchemeVersion> for EncryptionScheme {
    type Error = CommonError;
    fn try_from(value: EncryptionSchemeVersion) -> Result<Self> {
        match value {
            EncryptionSchemeVersion::Version1 => {
                Ok(Self::Version1(EncryptionSchemeVersion1::default()))
            }
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
        let key = Exactly32Bytes::generate();
        let msg = "Hello Radix";
        let msg_bytes = msg.bytes().collect();
        let encrypted = sut.encrypt(msg_bytes, &key);
        let decrypted_bytes = sut.decrypt(encrypted, &key).unwrap();
        let decrypted = String::from_utf8(decrypted_bytes).unwrap();
        assert_eq!(msg, decrypted);
    }

    #[test]
    fn decrypt_known() {
        let sut = SUT::default();
        let test = |encrypted_hex: &str,
                    key_hex: &str,
                    expected_plaintext: &str| {
            let decryption_key = Exactly32Bytes::from_str(key_hex).unwrap();
            let encrypted = hex_decode(encrypted_hex).unwrap();
            let decrypted = sut.decrypt(encrypted, &decryption_key).unwrap();
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
            sut.decrypt(Vec::new(), &Exactly32Bytes::sample()),
            Err(CommonError::InvalidAESBytesTooShort {
                expected_at_least: AESGCMSealedBox::LOWER_BOUND_LEN as u64,
                found: 0
            })
        );
    }
}

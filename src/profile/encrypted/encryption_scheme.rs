use crate::prelude::*;

use aes_gcm::{
    aead::{generic_array::sequence::Concat, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

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
    ) -> Result<Vec<u8>> {
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

#[repr(u32)]
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
pub enum EncryptionSchemeVersion {
    Version1 = 1,
}

#[derive(
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
pub struct EncryptionSchemeVersion1 {}
impl EncryptionSchemeVersion1 {
    pub const DESCRIPTION: &'static str = "AESGCM-256";
}

impl EncryptionSchemeVersion1 {
    fn seal(
        data: Vec<u8>,
        encryption_key: &Key<Aes256Gcm>,
    ) -> Result<AESSealedBox> {
        let cipher = Aes256Gcm::new(encryption_key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let cipher_text =
            cipher.encrypt(&nonce, data.as_ref()).map_err(|e| {
                error!("Failed to AES encrypt data - error: {:?}", e);
                CommonError::AESEncryptionFailed
            })?;
        let nonce = Exactly12Bytes::try_from(nonce.as_slice()).unwrap();

        Ok(AESSealedBox { nonce, cipher_text })
    }

    fn open(
        sealed_box: AESSealedBox,
        decryption_key: &Key<Aes256Gcm>,
    ) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(decryption_key);
        let cipher_text = sealed_box.cipher_text;
        cipher
            .decrypt(sealed_box.nonce.as_ref().into(), cipher_text.as_ref())
            .map_err(|e| {
                error!("Failed to AES decrypt data - error: {:?}", e);
                CommonError::AESDecryptionFailed
            })
    }
}

impl VersionOfAlgorithm for EncryptionSchemeVersion1 {
    type Version = EncryptionSchemeVersion;

    fn version(&self) -> Self::Version {
        Self::Version::Version1
    }

    fn description(&self) -> String {
        Self::DESCRIPTION.to_owned()
    }
}

impl VersionedEncryption for EncryptionSchemeVersion1 {
    fn encrypt(
        &self,
        data: Vec<u8>,
        encryption_key: &Exactly32Bytes,
    ) -> Result<Vec<u8>> {
        Self::seal(data, &Key::<Aes256Gcm>::from(*encryption_key))
            .map(|sb| sb.combined())
    }

    fn decrypt(
        &self,
        data: Vec<u8>,
        decryption_key: &Exactly32Bytes,
    ) -> Result<Vec<u8>> {
        let sealed_box = AESSealedBox::try_from(data)?;
        Self::open(sealed_box, &Key::<Aes256Gcm>::from(*decryption_key))
    }
}

impl From<Exactly32Bytes> for Key<Aes256Gcm> {
    fn from(value: Exactly32Bytes) -> Self {
        Self::from(*value.bytes())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct AESSealedBox {
    /// Nonce is 12 bytes
    nonce: Exactly12Bytes,

    /// Auth tag and encrypted payload
    cipher_text: Vec<u8>,
}
impl AESSealedBox {
    pub const AUTH_TAG_LEN: usize = 16;
    pub const NONCE_LEN: usize = 12;
    pub const LOWER_BOUND_LEN: usize = Self::AUTH_TAG_LEN + Self::NONCE_LEN + 1; // at least 1 byte cipher. VERY much LOWER bound

    fn combined(self) -> Vec<u8> {
        let mut combined = Vec::<u8>::new();
        let mut nonce = self.nonce.to_vec();
        let mut cipher_text = self.cipher_text;
        combined.append(&mut nonce);
        combined.append(&mut cipher_text);
        assert!(combined.len() >= Self::LOWER_BOUND_LEN);
        combined
    }
}

impl TryFrom<Vec<u8>> for AESSealedBox {
    type Error = CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < Self::LOWER_BOUND_LEN {
            return Err(CommonError::InvalidAESBytesTooShort {
                expected_at_least: Self::LOWER_BOUND_LEN as u64,
                found: value.len() as u64,
            });
        }

        let mut bytes = value;
        let nonce_bytes = bytes.drain(..Self::NONCE_LEN).collect_vec();
        let nonce = Exactly12Bytes::try_from(nonce_bytes).unwrap();
        Ok(Self {
            nonce,
            cipher_text: bytes,
        })
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
        let encrypted = sut.encrypt(msg_bytes, &key).unwrap();
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
}

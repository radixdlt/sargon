use crate::prelude::*;

use aes_gcm::{
    aead::{generic_array::sequence::Concat, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Key,
    Nonce, // Or `Aes128Gcm`
};

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
pub enum EncryptionScheme {
    Version1(EncryptionSchemeVersion1),
}
impl From<EncryptionSchemeVersion> for EncryptionScheme {
    fn from(value: EncryptionSchemeVersion) -> Self {
        match value {
            EncryptionSchemeVersion::Version1 => {
                Self::Version1(EncryptionSchemeVersion1::default())
            }
        }
    }
}
impl VersionedAlgorithm for EncryptionScheme {
    type Version = EncryptionSchemeVersion;

    fn version(&self) -> Self::Version {
        match self {
            EncryptionScheme::Version1(scheme) => scheme.version(),
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
    Serialize,
    Deserialize,
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
    Serialize,
    Deserialize,
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

    fn description(&self) -> String {
        Self::DESCRIPTION.to_owned()
    }
}
impl VersionedEncryption for EncryptionSchemeVersion1 {
    fn version(&self) -> EncryptionSchemeVersion {
        EncryptionSchemeVersion::Version1
    }

    fn encrypt(
        data: Vec<u8>,
        encryption_key: &Key<Aes256Gcm>,
    ) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(encryption_key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let cipher_text =
            cipher.encrypt(&nonce, data.as_ref()).map_err(|e| {
                error!("Failed to AES encrypt data - error: {:?}", e);
                CommonError::AESEncryptionFailed
            })?;
        let nonce = Exactly12Bytes::try_from(nonce.as_slice()).unwrap();

        let sealed_box = AESSealedBox { nonce, cipher_text };

        Ok(sealed_box.combined())
    }

    fn decrypt(
        data: Vec<u8>,
        decryption_key: &Key<Aes256Gcm>,
    ) -> Result<Vec<u8>> {
        let sealed_box = AESSealedBox::try_from(data)?;
        let cipher = Aes256Gcm::new(decryption_key);
        let cipher_text = sealed_box.cipher_text;
        cipher
            .decrypt(
                sealed_box.nonce.as_ref().try_into().unwrap(),
                cipher_text.as_ref(),
            )
            .map_err(|e| {
                error!("Failed to AES decrypt data - error: {:?}", e);
                CommonError::AESDecryptionFailed
            })
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
    type SUT = EncryptionSchemeVersion1;

    #[test]
    fn encryption_roundtrip() {
        let key = Aes256Gcm::generate_key(OsRng);
        let msg = "Hello Radix";
        let msg_bytes = msg.bytes().collect();
        let encrypted = SUT::encrypt(msg_bytes, &key).unwrap();
        let decrypted_bytes = SUT::decrypt(encrypted, &key).unwrap();
        let decrypted = String::from_utf8(decrypted_bytes).unwrap();
        assert_eq!(msg, decrypted);
    }
}

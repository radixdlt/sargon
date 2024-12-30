use crate::prelude::*;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Key,
};

/// AES GCM 256 encryption
#[derive(
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
pub struct AesGcm256 {}

impl HasSampleValues for AesGcm256 {
    fn sample() -> Self {
        Self::default()
    }

    fn sample_other() -> Self {
        Self::default()
    }
}

impl AesGcm256 {
    pub const DESCRIPTION: &'static str = "AESGCM-256";
}

impl AesGcm256 {
    fn seal(
        plaintext: impl AsRef<[u8]>,
        encryption_key: Key<aes_gcm::Aes256Gcm>,
    ) -> AesGcmSealedBox {
        let cipher = aes_gcm::Aes256Gcm::new(&encryption_key);
        let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut OsRng); // 12 bytes; unique per message
        let cipher_text = cipher
            .encrypt(&nonce, plaintext.as_ref())
            .expect("AES encrypt never fails for valid nonce.");
        let nonce = Exactly12Bytes::try_from(nonce.as_slice()).unwrap();

        AesGcmSealedBox { nonce, cipher_text }
    }

    fn open(
        sealed_box: AesGcmSealedBox,
        decryption_key: Key<aes_gcm::Aes256Gcm>,
    ) -> Result<Vec<u8>> {
        let cipher = aes_gcm::Aes256Gcm::new(&decryption_key);
        let cipher_text = sealed_box.cipher_text;
        cipher
            .decrypt(sealed_box.nonce.as_ref().into(), cipher_text.as_ref())
            .map_err(|e| {
                error!("Failed to AES decrypt data - error: {:?}", e);
                CommonError::AESDecryptionFailed
            })
    }
}

impl VersionOfAlgorithm for AesGcm256 {
    type Version = EncryptionSchemeVersion;

    fn version(&self) -> Self::Version {
        Self::Version::Version1
    }

    fn description(&self) -> String {
        Self::DESCRIPTION.to_owned()
    }
}

impl VersionedEncryption for AesGcm256 {
    /// Zeroizes `encryption_key` after usage.
    fn encrypt(
        &self,
        plaintext: impl AsRef<[u8]>,
        encryption_key: &mut EncryptionKey,
    ) -> Vec<u8> {
        let sealed_box = Self::seal(
            plaintext,
            Key::<aes_gcm::Aes256Gcm>::from(*encryption_key),
        );
        encryption_key.zeroize();
        sealed_box.combined()
    }

    /// Zeroizes `decryption_key` after usage.
    fn decrypt(
        &self,
        cipher_text: impl AsRef<[u8]>,
        decryption_key: &mut EncryptionKey,
    ) -> Result<Vec<u8>> {
        let sealed_box = AesGcmSealedBox::try_from(cipher_text.as_ref())?;
        let result = Self::open(
            sealed_box,
            Key::<aes_gcm::Aes256Gcm>::from(*decryption_key),
        );
        decryption_key.zeroize();
        result
    }
}

// impl From<Exactly32Bytes> for Key<aes_gcm::Aes256Gcm> {
//     fn from(value: Exactly32Bytes) -> Self {
//         Self::from(*value.bytes())
//     }
// }
impl From<EncryptionKey> for Key<aes_gcm::Aes256Gcm> {
    fn from(value: EncryptionKey) -> Self {
        Self::from(*value.0.bytes())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AesGcm256;

    #[test]
    fn test_fail() {
        assert_eq!(
            SUT::open(
                AesGcmSealedBox {
                    nonce: Exactly12Bytes::sample(),
                    cipher_text: hex_decode("deadbeef").unwrap(),
                },
                Key::<aes_gcm::Aes256Gcm>::from(
                    *Exactly32Bytes::sample_aced().bytes()
                )
            ),
            Err(CommonError::AESDecryptionFailed)
        )
    }
}

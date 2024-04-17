use crate::prelude::*;

use aes_gcm::{
    aead::{generic_array::sequence::Concat, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

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
    fn seal(data: Vec<u8>, encryption_key: &Key<Aes256Gcm>) -> AESGCMSealedBox {
        let cipher = Aes256Gcm::new(encryption_key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 12 bytes; unique per message
        let cipher_text = cipher
            .encrypt(&nonce, data.as_ref())
            .expect("AES encrypt never fails for valid nonce.");
        let nonce = Exactly12Bytes::try_from(nonce.as_slice()).unwrap();

        AESGCMSealedBox { nonce, cipher_text }
    }

    fn open(
        sealed_box: AESGCMSealedBox,
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
    ) -> Vec<u8> {
        let sealed_box =
            Self::seal(data, &Key::<Aes256Gcm>::from(*encryption_key));
        sealed_box.combined()
    }

    fn decrypt(
        &self,
        data: Vec<u8>,
        decryption_key: &Exactly32Bytes,
    ) -> Result<Vec<u8>> {
        let sealed_box = AESGCMSealedBox::try_from(data)?;
        Self::open(sealed_box, &Key::<Aes256Gcm>::from(*decryption_key))
    }
}

impl From<Exactly32Bytes> for Key<Aes256Gcm> {
    fn from(value: Exactly32Bytes) -> Self {
        Self::from(*value.bytes())
    }
}

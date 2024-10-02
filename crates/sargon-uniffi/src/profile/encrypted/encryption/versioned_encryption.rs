use crate::prelude::*;
pub trait VersionedEncryption: VersionOfAlgorithm {
    fn encrypt(
        &self,
        plaintext: impl AsRef<[u8]>,
        encryption_key: &mut EncryptionKey,
    ) -> Vec<u8>;

    fn decrypt(
        &self,
        cipher_text: impl AsRef<[u8]>,
        decryption_key: &mut EncryptionKey,
    ) -> Result<Vec<u8>>;
}

use crate::prelude::*;
pub trait VersionedEncryption: VersionOfAlgorithm {
    fn encrypt(
        &self,
        plaintext: Vec<u8>,
        encryption_key: &mut Exactly32Bytes,
    ) -> Vec<u8>;

    fn decrypt(
        &self,
        cipher_text: Vec<u8>,
        decryption_key: &mut Exactly32Bytes,
    ) -> Result<Vec<u8>>;
}

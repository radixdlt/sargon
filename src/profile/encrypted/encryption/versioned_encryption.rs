use crate::prelude::*;
pub trait VersionedEncryption: VersionOfAlgorithm {
    fn encrypt(
        &self,
        data: Vec<u8>,
        encryption_key: &Exactly32Bytes,
    ) -> Vec<u8>;

    fn decrypt(
        &self,
        data: Vec<u8>,
        decryption_key: &Exactly32Bytes,
    ) -> Result<Vec<u8>>;
}

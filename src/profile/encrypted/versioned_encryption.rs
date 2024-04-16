use crate::prelude::*;
pub trait VersionedEncryption {
    fn version(&self) -> EncryptionSchemeVersion;

    fn encrypt(
        data: Vec<u8>,
        encryption_key: &Exactly32Bytes,
    ) -> Result<Vec<u8>>;

    fn decrypt(
        data: Vec<u8>,
        decryption_key: &Exactly32Bytes,
    ) -> Result<Vec<u8>>;
}

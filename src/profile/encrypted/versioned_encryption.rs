use crate::prelude::*;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};

pub trait VersionedEncryption {
    fn version(&self) -> EncryptionSchemeVersion;
    fn encrypt(
        data: Vec<u8>,
        encryption_key: &Key<Aes256Gcm>,
    ) -> Result<Vec<u8>>;
    fn decrypt(
        data: Vec<u8>,
        decryption_key: &Key<Aes256Gcm>,
    ) -> Result<Vec<u8>>;
}

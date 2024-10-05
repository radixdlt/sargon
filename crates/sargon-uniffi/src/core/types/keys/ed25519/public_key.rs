use crate::{prelude::*, UniffiCustomTypeConverter};

use crypto::signatures::ed25519 as IotaSlip10Ed25519;

json_string_convertible!(Ed25519PublicKey);

/// An Ed25519 public key used to verify cryptographic signatures (EdDSA signatures).
#[serde_as]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
pub struct Ed25519PublicKey {
    secret_magic: ScryptoEd25519PublicKey,
}

uniffi::custom_type!(ScryptoEd25519PublicKey, BagOfBytes);
impl UniffiCustomTypeConverter for ScryptoEd25519PublicKey {
    type Builtin = BagOfBytes;

    #[cfg(not(tarpaulin_include))] // false negative | tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Self::try_from(val.as_slice()).map_err(|e| e.into())
    }

    #[cfg(not(tarpaulin_include))] // false negative | tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_vec().into()
    }
}

#[uniffi::export]
pub fn new_ed25519_public_key_from_hex(
    hex: String,
) -> Result<Ed25519PublicKey> {
    hex.parse()
}

#[uniffi::export]
pub fn new_ed25519_public_key_from_bytes(
    bytes: BagOfBytes,
) -> Result<Ed25519PublicKey> {
    bytes.to_vec().try_into()
}

#[uniffi::export]
pub fn new_ed25519_public_key_sample() -> Ed25519PublicKey {
    Ed25519PublicKey::sample()
}

#[uniffi::export]
pub fn new_ed25519_public_key_sample_other() -> Ed25519PublicKey {
    Ed25519PublicKey::sample_other()
}

#[uniffi::export]
pub fn android_secret_key_get_public_key_from_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
) -> Result<Ed25519PublicKey> {
    Ed25519PrivateKey::try_from(private_key_bytes.as_ref())
        .map(|k| k.public_key())
}

/// Encodes the `Ed25519PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde"`
#[uniffi::export]
pub fn ed25519_public_key_to_hex(public_key: &Ed25519PublicKey) -> String {
    public_key.to_hex()
}

#[uniffi::export]
pub fn ed25519_public_key_to_bytes(
    public_key: &Ed25519PublicKey,
) -> BagOfBytes {
    public_key.to_bytes().into()
}
use crate::prelude::*;
use sargon::Ed25519PublicKey as InternalEd25519PublicKey;

json_string_convertible!(Ed25519PublicKey);

/// An Ed25519 public key used to verify cryptographic signatures (EdDSA signatures).
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    
    
    uniffi::Record,
)]
pub struct Ed25519PublicKey {
    secret_magic: BagOfBytes,
}

impl From<InternalEd25519PublicKey> for Ed25519PublicKey {
    fn from(value: InternalEd25519PublicKey) -> Self {
        Self {
            secret_magic: value.to_bytes().into(),
        }
    }
}

impl Into<InternalEd25519PublicKey> for Ed25519PublicKey {
    fn into(self) -> InternalEd25519PublicKey {
        InternalEd25519PublicKey::try_from(self.secret_magic.to_vec()).unwrap()
    }
}

#[uniffi::export]
pub fn new_ed25519_public_key_from_hex(
    hex: String,
) -> Result<Ed25519PublicKey> {
    map_result_from_internal(hex.parse::<InternalEd25519PublicKey>())
}

#[uniffi::export]
pub fn new_ed25519_public_key_from_bytes(
    bytes: BagOfBytes,
) -> Result<Ed25519PublicKey> {
    map_result_from_internal(bytes.to_vec().try_into::<InternalEd25519PublicKey>())
}

#[uniffi::export]
pub fn new_ed25519_public_key_sample() -> Ed25519PublicKey {
    InternalEd25519PublicKey::sample().into()
}

#[uniffi::export]
pub fn new_ed25519_public_key_sample_other() -> Ed25519PublicKey {
    InternalEd25519PublicKey::sample_other().into()
}

#[uniffi::export]
pub fn android_secret_key_get_public_key_from_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
) -> Result<Ed25519PublicKey> {
    map_result_from_internal(InternalEd25519PublicKey::from_private_key_bytes(private_key_bytes.into()))
}

/// Encodes the `Ed25519PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde"`
#[uniffi::export]
pub fn ed25519_public_key_to_hex(public_key: &Ed25519PublicKey) -> String {
    public_key.into::<InternalEd25519PublicKey>().to_hex()
}

#[uniffi::export]
pub fn ed25519_public_key_to_bytes(
    public_key: &Ed25519PublicKey,
) -> BagOfBytes {
    public_key.into::<InternalEd25519PublicKey>().to_bag_of_bytes().into()
}
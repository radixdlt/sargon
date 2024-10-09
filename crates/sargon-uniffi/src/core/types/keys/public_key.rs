use crate::prelude::*;
use sargon::PublicKey as InternalPublicKey;
use sargon::BagOfBytes as InternalBagOfBytes;

/// A tagged union of supported public keys on different curves, supported
/// curves are `secp256k1` and `Curve25519`
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
    uniffi::Enum,
)]
pub enum PublicKey {
    /// An Ed25519 public key used to verify cryptographic signatures.
    Ed25519(Ed25519PublicKey),

    /// A secp256k1 public key used to verify cryptographic signatures (ECDSA signatures).
    Secp256k1(Secp256k1PublicKey),
}

impl From<InternalPublicKey> for PublicKey {
    fn from(value: InternalPublicKey) -> Self {
        match value {
            InternalPublicKey::Ed25519(value) => Self::Ed25519(value.into()),
            InternalPublicKey::Secp256k1(value) => Self::Secp256k1(value.into()),
        }
    }
}

impl Into<InternalPublicKey> for PublicKey {
    fn into(self) -> InternalPublicKey {
        match self {
            PublicKey::Ed25519(value) => InternalPublicKey::Ed25519(value.into()),
            PublicKey::Secp256k1(value) => InternalPublicKey::Secp256k1(value.into()),
        }
    }
}

/// Tries to create a PublicKey from the hex string
#[uniffi::export]
pub fn new_public_key_from_hex(hex: String) -> Result<PublicKey> {
    InternalPublicKey::from_str(&hex).map_result()
}

/// Tries to create a PublicKey from the bytes
#[uniffi::export]
pub fn new_public_key_from_bytes(
    bag_of_bytes: BagOfBytes,
) -> Result<PublicKey> {
    InternalPublicKey::try_from(bag_of_bytes.into()).map_result()
}

#[uniffi::export]
pub fn public_key_to_hex(public_key: &PublicKey) -> String {
    public_key.into_internal().to_hex()
}

#[uniffi::export]
pub fn public_key_to_bytes(public_key: &PublicKey) -> BagOfBytes {
    public_key.into_internal().to_bytes().into()
}

#[uniffi::export]
pub fn new_public_key_sample() -> PublicKey {
    InternalPublicKey::sample().into()
}

#[uniffi::export]
pub fn new_public_key_sample_other() -> PublicKey {
    InternalPublicKey::sample_other().into()
}

/// Verifies an Elliptic Curve signature over either Curve25519 or Secp256k1
#[uniffi::export]
pub fn public_key_is_valid_signature_for_hash(
    public_key: &PublicKey,
    signature: Signature,
    hash: Hash,
) -> bool {
    public_key.into_internal().is_valid_signature_for_hash(signature.into(), &hash.into())
}
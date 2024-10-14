use crate::{prelude::*, UniffiCustomTypeConverter};
use sargon::BagOfBytes as InternalBagOfBytes;
use sargon::Secp256k1PublicKey as InternalSecp256k1PublicKey;

/// A `secp256k1` public key used to verify cryptographic signatures (ECDSA signatures).
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct Secp256k1PublicKey {
    secret_magic: BagOfBytes,
}

impl Secp256k1PublicKey {
    pub fn into_internal(&self) -> InternalSecp256k1PublicKey {
        self.clone().into()
    }
}

impl From<InternalSecp256k1PublicKey> for Secp256k1PublicKey {
    fn from(value: InternalSecp256k1PublicKey) -> Self {
        Self {
            secret_magic: value.to_bag_of_bytes().into(),
        }
    }
}

impl Into<InternalSecp256k1PublicKey> for Secp256k1PublicKey {
    fn into(self) -> InternalSecp256k1PublicKey {
        InternalSecp256k1PublicKey::try_from(self.secret_magic.to_vec()).unwrap()
    }
}

#[uniffi::export]
pub fn new_secp256k1_public_key_from_hex(
    hex: String,
) -> Result<Secp256k1PublicKey> {
    hex.parse::<InternalSecp256k1PublicKey>().into_result()
}

/// Creates a Secp256k1PublicKey from either compressed form (33 bytes) or
/// from uncompressed form (65 bytes).
#[uniffi::export]
pub fn new_secp256k1_public_key_from_bytes(
    bytes: BagOfBytes,
) -> Result<Secp256k1PublicKey> {
    InternalSecp256k1PublicKey::try_from(bytes.into_internal().to_vec())
        .into_result()
}

/// Encodes the compressed form (33 bytes) of a `Secp256k1PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"`
#[uniffi::export]
pub fn secp256k1_public_key_to_hex(public_key: &Secp256k1PublicKey) -> String {
    public_key.into_internal().to_hex()
}

/// Returns the public key on **compressed** form (33 bytes)
#[uniffi::export]
pub fn secp256k1_public_key_to_bytes(
    public_key: &Secp256k1PublicKey,
) -> BagOfBytes {
    public_key.secret_magic.clone()
}

/// Returns the public key on **uncompressed** form (65 bytes)
#[uniffi::export]
pub fn secp256k1_public_key_to_bytes_uncompressed(
    public_key: &Secp256k1PublicKey,
) -> BagOfBytes {
    InternalBagOfBytes::from(public_key.into_internal().uncompressed()).into()
}

#[uniffi::export]
pub fn new_secp256k1_public_key_sample() -> Secp256k1PublicKey {
    InternalSecp256k1PublicKey::sample().into()
}

#[uniffi::export]
pub fn new_secp256k1_public_key_sample_other() -> Secp256k1PublicKey {
    InternalSecp256k1PublicKey::sample_other().into()
}

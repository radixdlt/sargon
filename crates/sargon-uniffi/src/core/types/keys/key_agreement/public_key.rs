use crate::prelude::*;
use sargon::KeyAgreementPublicKey as InternalKeyAgreementPublicKey;

/// PublicKey on Curve25519 used for key agreement (ECDH) with some `KeyAgreementPrivateKey`.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct KeyAgreementPublicKey {
    pub value: BagOfBytes,
}

delegate_display_debug_into!(
    KeyAgreementPublicKey,
    InternalKeyAgreementPublicKey
);

impl From<InternalKeyAgreementPublicKey> for KeyAgreementPublicKey {
    fn from(value: InternalKeyAgreementPublicKey) -> Self {
        Self {
            value: value.to_bytes().into(),
        }
    }
}

impl Into<InternalKeyAgreementPublicKey> for KeyAgreementPublicKey {
    fn into(self) -> InternalKeyAgreementPublicKey {
        InternalKeyAgreementPublicKey::try_from(
            self.value.into_internal().to_vec(),
        )
        .unwrap()
    }
}

#[uniffi::export]
pub fn new_key_agreement_public_key_from_hex(
    hex: String,
) -> Result<KeyAgreementPublicKey> {
    hex.parse::<InternalKeyAgreementPublicKey>().map_result()
}

/// Creates a Secp256k1PublicKey from either compressed form (33 bytes) or
/// from uncompressed form (65 bytes).
#[uniffi::export]
pub fn new_key_agreement_public_key_from_bytes(
    bytes: BagOfBytes,
) -> Result<KeyAgreementPublicKey> {
    InternalKeyAgreementPublicKey::try_from(bytes.into_internal().to_vec())
        .map_result()
}

/// Encodes the compressed form (33 bytes) of a `Secp256k1PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"`
#[uniffi::export]
pub fn key_agreement_public_key_to_hex(
    public_key: &KeyAgreementPublicKey,
) -> String {
    public_key.into_internal().to_hex()
}

/// Returns the public key on **compressed** form (33 bytes)
#[uniffi::export]
pub fn key_agreement_public_key_to_bytes(
    public_key: &KeyAgreementPublicKey,
) -> BagOfBytes {
    public_key.value.clone()
}

#[uniffi::export]
pub fn new_key_agreement_public_key_sample() -> KeyAgreementPublicKey {
    InternalKeyAgreementPublicKey::sample().into()
}

#[uniffi::export]
pub fn new_key_agreement_public_key_sample_other() -> KeyAgreementPublicKey {
    InternalKeyAgreementPublicKey::sample_other().into()
}

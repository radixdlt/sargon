use crate::prelude::*;

#[uniffi::export]
pub fn new_key_agreement_public_key_from_hex(
    hex: String,
) -> Result<KeyAgreementPublicKey> {
    hex.parse()
}

/// Creates a Secp256k1PublicKey from either compressed form (33 bytes) or
/// from uncompressed form (65 bytes).
#[uniffi::export]
pub fn new_key_agreement_public_key_from_bytes(
    bytes: BagOfBytes,
) -> Result<KeyAgreementPublicKey> {
    KeyAgreementPublicKey::try_from(bytes.to_vec())
}

/// Encodes the compressed form (33 bytes) of a `Secp256k1PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"`
#[uniffi::export]
pub fn key_agreement_public_key_to_hex(
    public_key: &KeyAgreementPublicKey,
) -> String {
    public_key.to_hex()
}

/// Returns the public key on **compressed** form (33 bytes)
#[uniffi::export]
pub fn key_agreeement_public_key_to_bytes(
    public_key: &KeyAgreementPublicKey,
) -> BagOfBytes {
    BagOfBytes::from(public_key.to_bytes())
}

#[uniffi::export]
pub fn new_key_agreement_public_key_sample() -> KeyAgreementPublicKey {
    KeyAgreementPublicKey::sample()
}

#[uniffi::export]
pub fn new_key_agreement_public_key_sample_other() -> KeyAgreementPublicKey {
    KeyAgreementPublicKey::sample_other()
}

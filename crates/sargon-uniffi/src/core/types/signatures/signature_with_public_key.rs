use crate::prelude::*;
use sargon::SignatureWithPublicKey as InternalSignatureWithPublicKey;

/// Represents any natively supported signature, including public key.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum SignatureWithPublicKey {
    // N.B. `radix_transactions::model::SignatureWithPublicKeyV1::Secp256k1` does
    // NOT include the public key, it relies on ECDSA Signature supporting
    // recovery, but it is not reliable since passing the wrong hash to
    // a signature will return the WRONG public key. In other words one might
    // naively believe that recovery should fail for the wrong hash passed in,
    // but instead the wrong public key is returned. In the context of Scrypto
    // or Node, they might have a mechanism by which they can validate the
    // public key against some address or sub-state, but we play it safe, the
    // cost of having the public key around in the ephemeral operations working
    // with `SignatureWithPublicKey` is near-zero, so we have it explicit in state.
    Secp256k1 {
        public_key: Secp256k1PublicKey,
        signature: Secp256k1Signature,
    },
    Ed25519 {
        public_key: Ed25519PublicKey,
        signature: Ed25519Signature,
    },
}

#[uniffi::export]
pub fn new_signature_with_public_key_sample() -> SignatureWithPublicKey {
    InternalSignatureWithPublicKey::sample().into()
}

#[uniffi::export]
pub fn new_signature_with_public_key_sample_other() -> SignatureWithPublicKey {
    InternalSignatureWithPublicKey::sample_other().into()
}

#[uniffi::export]
pub fn signature_with_public_key_get_public_key(
    signature_with_public_key: &SignatureWithPublicKey,
) -> PublicKey {
    signature_with_public_key
        .into_internal()
        .public_key()
        .into()
}

#[uniffi::export]
pub fn signature_with_public_key_get_signature(
    signature_with_public_key: &SignatureWithPublicKey,
) -> Signature {
    signature_with_public_key.into_internal().signature().into()
}

#[uniffi::export]
pub fn signature_with_public_key_is_valid(
    signature_with_public_key: &SignatureWithPublicKey,
    for_hash: &Hash,
) -> bool {
    signature_with_public_key
        .into_internal()
        .is_valid_for_hash(&for_hash.into_internal())
        .into()
}

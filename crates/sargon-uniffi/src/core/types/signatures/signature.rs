use crate::prelude::*;
use sargon::Signature as InternalSignature;

/// Either a Signature on `Curve25519` or `Secp256k1`
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum Signature {
    Secp256k1 { value: Secp256k1Signature },
    Ed25519 { value: Ed25519Signature },
}

#[uniffi::export]
pub fn new_signature_sample() -> Signature {
    InternalSignature::sample().into()
}

#[uniffi::export]
pub fn new_signature_sample_other() -> Signature {
    InternalSignature::sample_other().into()
}

#[uniffi::export]
pub fn new_signature_from_bytes(bytes: BagOfBytes) -> Result<Signature> {
    InternalSignature::try_from(bytes.into_internal()).map_result()
}

#[uniffi::export]
pub fn signature_to_string(signature: &Signature) -> String {
    signature.into_internal().to_string()
}

#[uniffi::export]
pub fn signature_to_bytes(signature: &Signature) -> BagOfBytes {
    signature.into_internal().to_bytes().into()
}

use crate::prelude::*;
use sargon::Secp256k1Signature as InternalSecp256k1Signature;

/// Represents an Secp256k1 signature.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct Secp256k1Signature {
    // recovery id + signature
    pub bytes: Exactly65Bytes,
}

impl From<InternalSecp256k1Signature> for Secp256k1Signature {
    fn from(value: InternalSecp256k1Signature) -> Self {
        Self {
            bytes: value.bytes.into(),
        }
    }
}

impl Into<InternalSecp256k1Signature> for Secp256k1Signature {
    fn into(self) -> InternalSecp256k1Signature {
        InternalSecp256k1Signature {
            bytes: self.bytes.into(),
        }
    }
}

#[uniffi::export]
pub fn new_secp256k1_signature_sample() -> Secp256k1Signature {
    InternalSecp256k1Signature::sample().into()
}

#[uniffi::export]
pub fn new_secp256k1_signature_sample_other() -> Secp256k1Signature {
    InternalSecp256k1Signature::sample_other().into()
}

#[uniffi::export]
pub fn new_secp256k1_signature_from_exactly_65_bytes(
    bytes: Exactly65Bytes,
) -> Secp256k1Signature {
    InternalSecp256k1Signature::from(bytes.into_internal()).into()
}

#[uniffi::export]
pub fn new_secp256k1_signature_from_bytes(
    bytes: BagOfBytes,
) -> Result<Secp256k1Signature> {
    InternalSecp256k1Signature::try_from(bytes.into_internal()).map_result()
}

#[uniffi::export]
pub fn secp256k1_signature_to_string(signature: &Secp256k1Signature) -> String {
    signature.into_internal().to_string()
}

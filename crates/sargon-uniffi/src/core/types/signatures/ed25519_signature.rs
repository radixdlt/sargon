use crate::prelude::*;
use sargon::Ed25519Signature as InternalEd25519Signature;

json_string_convertible!(Ed25519Signature);

/// Represents an ED25519 signature.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct Ed25519Signature {
    pub bytes: Exactly64Bytes,
}

impl From<InternalEd25519Signature> for Ed25519Signature {
    fn from(value: InternalEd25519Signature) -> Self {
        Self {
            bytes: value.bytes.into(),
        }
    }
}

impl Into<InternalEd25519Signature> for Ed25519Signature {
    fn into(self) -> InternalEd25519Signature {
        InternalEd25519Signature {
            bytes: self.bytes.into(),
        }
    }
}

#[uniffi::export]
pub fn new_ed25519_signature_sample() -> Ed25519Signature {
    InternalEd25519Signature::sample().into()
}

#[uniffi::export]
pub fn new_ed25519_signature_sample_other() -> Ed25519Signature {
    InternalEd25519Signature::sample_other().into()
}

#[uniffi::export]
pub fn new_ed25519_signature_from_exactly_64_bytes(
    bytes: Exactly64Bytes,
) -> Ed25519Signature {
    InternalEd25519Signature::from(bytes.into_internal()).into()
}

#[uniffi::export]
pub fn new_ed25519_signature_from_bytes(
    bytes: BagOfBytes,
) -> Result<Ed25519Signature> {
    InternalEd25519Signature::try_from(bytes.into_internal()).map_result()
}

#[uniffi::export]
pub fn ed25519_signature_to_string(signature: &Ed25519Signature) -> String {
    signature.into_internal().to_string()
}

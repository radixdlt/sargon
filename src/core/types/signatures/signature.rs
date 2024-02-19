use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Enum,
)]
pub enum Signature {
    Secp256k1 { value: Secp256k1Signature },
    Ed25519 { value: Ed25519Signature },
}

impl From<Secp256k1Signature> for Signature {
    fn from(signature: Secp256k1Signature) -> Self {
        Self::Secp256k1 { value: signature }
    }
}

impl From<Ed25519Signature> for Signature {
    fn from(signature: Ed25519Signature) -> Self {
        Self::Ed25519 { value: signature }
    }
}

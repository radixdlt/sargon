// use crate::prelude::*;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum SignatureV1 {
//     Secp256k1(Secp256k1Signature),
//     Ed25519(Ed25519Signature),
// }

// impl From<Secp256k1Signature> for SignatureV1 {
//     fn from(signature: Secp256k1Signature) -> Self {
//         Self::Secp256k1(signature)
//     }
// }

// impl From<Ed25519Signature> for SignatureV1 {
//     fn from(signature: Ed25519Signature) -> Self {
//         Self::Ed25519(signature)
//     }
// }

// #[derive(Debug, Clone, Eq, PartialEq, ManifestSbor)]
// #[sbor(transparent)]
// pub struct NotarySignatureV1(pub SignatureV1);

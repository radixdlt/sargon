// use crate::prelude::*;

// use radix_engine_common::crypto::Ed25519Signature as ScryptoEd25519Signature;
// use radix_engine_common::crypto::Secp256k1Signature as ScryptoSecp256k1Signature;

// /// Represents any natively supported signature, including public key.
// #[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum, EnumAsInner)]
// pub enum SignatureWithPublicKeyV1 {
//     Secp256k1 {
//         signature: ScryptoSecp256k1Signature,
//     },
//     Ed25519 {
//         public_key: Ed25519PublicKey,
//         signature: ScryptoEd25519Signature,
//     },
// }

// impl SignatureWithPublicKeyV1 {
//     pub fn signature(&self) -> SignatureV1 {
//         match &self {
//             Self::Secp256k1 { signature } => signature.clone().into(),
//             Self::Ed25519 { signature, .. } => signature.clone().into(),
//         }
//     }
// }

// impl From<Secp256k1Signature> for SignatureWithPublicKeyV1 {
//     fn from(signature: Secp256k1Signature) -> Self {
//         Self::Secp256k1 { signature }
//     }
// }

// impl From<(Ed25519PublicKey, Ed25519Signature)> for SignatureWithPublicKeyV1 {
//     fn from(
//         (public_key, signature): (Ed25519PublicKey, Ed25519Signature),
//     ) -> Self {
//         Self::Ed25519 {
//             public_key,
//             signature,
//         }
//     }
// }

use crate::prelude::*;

use radix_engine_common::crypto::Ed25519Signature as ScryptoEd25519Signature;
use radix_engine_common::crypto::Secp256k1Signature as ScryptoSecp256k1Signature;
use transaction::model::SignatureWithPublicKeyV1 as ScryptoSignatureWithPublicKey;

/// Represents any natively supported signature, including public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum, EnumAsInner)]
pub enum SignatureWithPublicKey {
    Secp256k1 {
        signature: Secp256k1Signature,
    },
    Ed25519 {
        public_key: Ed25519PublicKey,
        signature: Ed25519Signature,
    },
}

impl From<ScryptoSignatureWithPublicKey> for SignatureWithPublicKey {
    fn from(value: ScryptoSignatureWithPublicKey) -> Self {
        match value {
            ScryptoSignatureWithPublicKey::Secp256k1 { signature } => {
                Self::Secp256k1 {
                    signature: signature.into(),
                }
            }
            ScryptoSignatureWithPublicKey::Ed25519 {
                public_key,
                signature,
            } => Self::Ed25519 {
                public_key: public_key
                    .try_into()
                    .expect("Invalid public key found."),
                signature: signature.into(),
            },
        }
    }
}
impl From<SignatureWithPublicKey> for ScryptoSignatureWithPublicKey {
    fn from(value: SignatureWithPublicKey) -> Self {
        match value {
            SignatureWithPublicKey::Secp256k1 { signature } => {
                Self::Secp256k1 {
                    signature: signature.into(),
                }
            }
            SignatureWithPublicKey::Ed25519 {
                public_key,
                signature,
            } => Self::Ed25519 {
                public_key: public_key.into(),
                signature: signature.into(),
            },
        }
    }
}

impl SignatureWithPublicKey {
    pub fn signature(&self) -> Signature {
        match &self {
            Self::Secp256k1 { signature } => signature.clone().into(),
            Self::Ed25519 { signature, .. } => signature.clone().into(),
        }
    }
}

impl From<Secp256k1Signature> for SignatureWithPublicKey {
    fn from(signature: Secp256k1Signature) -> Self {
        Self::Secp256k1 { signature }
    }
}

impl From<(Ed25519PublicKey, Ed25519Signature)> for SignatureWithPublicKey {
    fn from(
        (public_key, signature): (Ed25519PublicKey, Ed25519Signature),
    ) -> Self {
        Self::Ed25519 {
            public_key,
            signature,
        }
    }
}

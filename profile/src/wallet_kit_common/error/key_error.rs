use crate::prelude::*;

#[derive(Clone, Debug, ThisError, PartialEq, uniffi::Error)]
#[uniffi(flat_error)]
pub enum KeyError {
    #[error("Failed to create Ed25519 Private key from bytes.")]
    InvalidEd25519PrivateKeyFromBytes,

    #[error("Failed to create Ed25519 Private key from String.")]
    InvalidEd25519PrivateKeyFromString,

    #[error("Failed to create Secp256k1 Private key from bytes.")]
    InvalidSecp256k1PrivateKeyFromBytes,

    #[error("Failed to create Secp256k1 Private key from String.")]
    InvalidSecp256k1PrivateKeyFromString,

    #[error("Failed to create Ed25519 Public key from bytes.")]
    InvalidEd25519PublicKeyFromBytes,

    #[error("Failed to create Ed25519 Public key from String.")]
    InvalidEd25519PublicKeyFromString,

    #[error("Failed to create Secp256k1 Public key from bytes.")]
    InvalidSecp256k1PublicKeyFromBytes,

    #[error("Failed to create Secp256k1 Public key from String.")]
    InvalidSecp256k1PublicKeyFromString,

    #[error("Failed to create Secp256k1 Public key, invalid point, not on curve.")]
    InvalidSecp256k1PublicKeyPointNotOnCurve,

    #[error("Failed to create Ed25519 Public key, invalid point, not on curve.")]
    InvalidEd25519PublicKeyPointNotOnCurve,
}

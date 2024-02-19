use crate::prelude::*;

use radix_engine_common::crypto::Ed25519Signature as ScryptoEd25519Signature;

/// Represents an ED25519 signature.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    derive_more::FromStr,
    uniffi::Record,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Ed25519Signature {
    pub bytes: Hex64Bytes,
}

impl From<ScryptoEd25519Signature> for Ed25519Signature {
    fn from(value: ScryptoEd25519Signature) -> Self {
        Self {
            bytes: Hex64Bytes::from_bytes(&value.0),
        }
    }
}

impl From<Ed25519Signature> for ScryptoEd25519Signature {
    fn from(value: Ed25519Signature) -> Self {
        ScryptoEd25519Signature(value.bytes.bytes())
    }
}

impl Ed25519Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }
}

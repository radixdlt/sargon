use crate::prelude::*;

use radix_engine_common::crypto::Secp256k1Signature as ScryptoSecp256k1Signature;

/// Represents an Secp256k1 signature.
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
pub struct Secp256k1Signature {
    // recovery id + signature
    pub bytes: Hex65Bytes,
}

impl From<ScryptoSecp256k1Signature> for Secp256k1Signature {
    fn from(value: ScryptoSecp256k1Signature) -> Self {
        Self {
            bytes: Hex65Bytes::from_bytes(&value.0),
        }
    }
}

impl From<Secp256k1Signature> for ScryptoSecp256k1Signature {
    fn from(value: Secp256k1Signature) -> Self {
        ScryptoSecp256k1Signature(value.bytes.bytes())
    }
}

impl Secp256k1Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }
}

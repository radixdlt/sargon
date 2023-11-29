use crate::{
    error::Error,
    types::{hex_32bytes::Hex32Bytes, keys::ed25519::private_key::Ed25519PrivateKey},
};
use radix_engine_common::crypto::{Ed25519PublicKey as EngineEd25519PublicKey, Hash};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};
use transaction::{signing::ed25519::Ed25519Signature, validation::verify_ed25519};

/// An Ed25519 public key used to verify cryptographic signatures.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ed25519PublicKey(EngineEd25519PublicKey);

impl Ed25519PublicKey {
    pub(crate) fn from_engine(engine: EngineEd25519PublicKey) -> Result<Self, Error> {
        ed25519_dalek::PublicKey::from_bytes(engine.to_vec().as_slice())
            .map(|_| Self(engine))
            .map_err(|_| Error::InvalidEd25519PublicKeyPointNotOnCurve)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// Verifies an EdDSA signature over Curve25519.
    pub fn is_valid(&self, signature: &Ed25519Signature, for_hash: &Hash) -> bool {
        verify_ed25519(for_hash, &self.0, signature)
    }
}

impl TryFrom<&[u8]> for Ed25519PublicKey {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> Result<Ed25519PublicKey, Self::Error> {
        EngineEd25519PublicKey::try_from(slice)
            .map_err(|_| Error::InvalidEd25519PublicKeyFromBytes)
            .and_then(|pk| Self::from_engine(pk))
    }
}

impl TryInto<Ed25519PublicKey> for &str {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Ed25519PublicKey, Self::Error> {
        Ed25519PublicKey::from_str(self)
    }
}

impl Ed25519PublicKey {
    pub fn from_str(hex: &str) -> Result<Self, Error> {
        Hex32Bytes::from_str(hex)
            .map_err(|_| Error::InvalidEd25519PublicKeyFromString)
            .and_then(|b| Ed25519PublicKey::try_from(b.to_vec().as_slice()))
    }
}

impl Debug for Ed25519PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl Ed25519PublicKey {
    pub fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    pub fn placeholder_alice() -> Self {
        Ed25519PrivateKey::placeholder_alice().public_key()
    }

    pub fn placeholder_bob() -> Self {
        Ed25519PrivateKey::placeholder_bob().public_key()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::{error::Error, json::assert_json_value_eq_after_roundtrip};
    use serde_json::json;

    use super::Ed25519PublicKey;
    #[test]
    fn json() {
        let model = Ed25519PublicKey::placeholder_alice();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"),
        )
    }

    #[test]
    fn from_str() {
        assert!(Ed25519PublicKey::from_str(
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        )
        .is_ok());
    }

    #[test]
    fn bytes_roundtrip() {
        let bytes: &[u8] = &[
            0xec, 0x17, 0x2b, 0x93, 0xad, 0x5e, 0x56, 0x3b, 0xf4, 0x93, 0x2c, 0x70, 0xe1, 0x24,
            0x50, 0x34, 0xc3, 0x54, 0x67, 0xef, 0x2e, 0xfd, 0x4d, 0x64, 0xeb, 0xf8, 0x19, 0x68,
            0x34, 0x67, 0xe2, 0xbf,
        ];
        let key = Ed25519PublicKey::try_from(bytes).unwrap();
        assert_eq!(
            key.to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn placeholder_alice() {
        assert_eq!(
            Ed25519PublicKey::placeholder_alice().to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
    }

    #[test]
    fn placeholder_bob() {
        assert_eq!(
            Ed25519PublicKey::placeholder_bob().to_hex(),
            "b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde"
        );
    }

    #[test]
    fn invalid_bytes() {
        assert_eq!(
            Ed25519PublicKey::try_from(&[0u8] as &[u8]),
            Err(Error::InvalidEd25519PublicKeyFromBytes)
        );
    }

    #[test]
    fn invalid_hex_str() {
        assert_eq!(
            Ed25519PublicKey::from_str("not a valid hex string"),
            Err(Error::InvalidEd25519PublicKeyFromString)
        );
    }

    #[test]
    fn invalid_str_too_short() {
        assert_eq!(
            Ed25519PublicKey::from_str("dead"),
            Err(Error::InvalidEd25519PublicKeyFromString)
        );
    }

    #[test]
    fn invalid_key_not_on_curve() {
        assert_eq!(
            Ed25519PublicKey::from_str(
                "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
            ),
            Err(Error::InvalidEd25519PublicKeyPointNotOnCurve)
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Ed25519PublicKey::placeholder_alice()),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            Ed25519PublicKey::placeholder_alice(),
            Ed25519PublicKey::placeholder_bob()
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            Ed25519PublicKey::placeholder_alice(),
            Ed25519PublicKey::placeholder_alice()
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([
                Ed25519PublicKey::placeholder_alice(),
                Ed25519PublicKey::placeholder_alice()
            ])
            .len(),
            1
        );
    }
}

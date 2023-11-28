use crate::{error::Error, types::keys::ed25519::private_key::Ed25519PrivateKey};
use radix_engine_common::crypto::Ed25519PublicKey as EngineEd25519PublicKey;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ed25519PublicKey(EngineEd25519PublicKey);

impl Ed25519PublicKey {
    pub const LENGTH: usize = 32;

    pub(crate) fn from_engine(engine: EngineEd25519PublicKey) -> Self {
        Self(engine)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    // pub fn to_hash(&self) -> Ed25519PublicKeyHash {
    //     Ed25519PublicKeyHash::new_from_public_key(self)
    // }
}

impl Debug for Ed25519PublicKey {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ed25519PublicKey")
            .field("hex", &self.to_hex())
            .finish()
    }
}

impl TryFrom<&[u8]> for Ed25519PublicKey {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        EngineEd25519PublicKey::try_from(slice)
            .map(Ed25519PublicKey)
            .map_err(|_| Error::InvalidEd25519PublicKeyFromBytes)
    }
}

impl Ed25519PublicKey {
    pub fn placeholder() -> Self {
        Self::placeholder_alice()
    }
    pub fn placeholder_alice() -> Self {
        let private_key = Ed25519PrivateKey::placeholder_alice();
        let public_key = private_key.public_key();
        assert_eq!(
            public_key.to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
        return public_key;
    }
    pub fn placeholder_bob() -> Self {
        let private_key = Ed25519PrivateKey::placeholder_bob();
        let public_key = private_key.public_key();
        assert_eq!(
            public_key.to_hex(),
            "b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde"
        );
        return public_key;
    }
}

#[cfg(test)]
mod tests {
    use crate::json::assert_json_value_eq_after_roundtrip;
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
}

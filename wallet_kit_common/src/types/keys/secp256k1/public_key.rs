use crate::{error::Error, types::keys::secp256k1::private_key::Secp256k1PrivateKey};
use radix_engine_common::crypto::{
    Hash, Secp256k1PublicKey as EngineSecp256k1PublicKey, Secp256k1PublicKeyHash,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use transaction::{signing::secp256k1::Secp256k1Signature, validation::verify_secp256k1};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Secp256k1PublicKey(EngineSecp256k1PublicKey);

impl Secp256k1PublicKey {
    pub const LENGTH: usize = 33;

    pub(crate) fn from_engine(engine: EngineSecp256k1PublicKey) -> Self {
        Self(engine)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// Verifies an ECDSA signature over Secp256k1.
    pub fn is_valid(&self, signature: &Secp256k1Signature, for_hash: &Hash) -> bool {
        verify_secp256k1(for_hash, &self.0, signature)
    }

    pub fn to_hash(&self) -> Secp256k1PublicKeyHash {
        Secp256k1PublicKeyHash::new_from_public_key(&self.0)
    }
}

impl TryFrom<&[u8]> for Secp256k1PublicKey {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        EngineSecp256k1PublicKey::try_from(slice)
            .map(Secp256k1PublicKey)
            .map_err(|_| Error::InvalidSecp256k1PublicKeyFromBytes)
    }
}

impl TryInto<Secp256k1PublicKey> for &str {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Secp256k1PublicKey, Self::Error> {
        Secp256k1PublicKey::from_str(self)
    }
}

impl Secp256k1PublicKey {
    pub fn from_str(hex: &str) -> Result<Self, Error> {
        hex::decode(hex)
            .map_err(|_| Error::InvalidSecp256k1PublicKeyFromString)
            .and_then(|b| Secp256k1PublicKey::try_from(b.as_slice()))
    }
}

impl Debug for Secp256k1PublicKey {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secp256k1PublicKey")
            .field("compressed_hex", &self.to_hex())
            .finish()
    }
}

impl Secp256k1PublicKey {
    pub fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    pub fn placeholder_alice() -> Self {
        let private_key = Secp256k1PrivateKey::placeholder_alice();
        let public_key = private_key.public_key();
        assert_eq!(
            public_key.to_hex(),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
        return public_key;
    }

    pub fn placeholder_bob() -> Self {
        let private_key = Secp256k1PrivateKey::placeholder_bob();
        let public_key = private_key.public_key();
        assert_eq!(
            public_key.to_hex(),
            "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"
        );
        return public_key;
    }
}

#[cfg(test)]
mod tests {
    use super::Secp256k1PublicKey;
    use crate::json::assert_json_value_eq_after_roundtrip;
    use serde_json::json;
    #[test]
    fn json() {
        let model = Secp256k1PublicKey::placeholder_bob();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"),
        )
    }
}

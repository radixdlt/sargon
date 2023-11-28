use radix_engine_common::crypto::Secp256k1PublicKey as EngineSecp256k1PublicKey;

use crate::error::Error;

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

    // pub fn to_hash(&self) -> Secp256k1PublicKeyHash {
    //     Secp256k1PublicKeyHash::new_from_public_key(self)
    // }
}

impl TryFrom<&[u8]> for Secp256k1PublicKey {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        EngineSecp256k1PublicKey::try_from(slice)
            .map(Secp256k1PublicKey)
            .map_err(|_| Error::InvalidSecp256k1PublicKeyFromBytes)
    }
}

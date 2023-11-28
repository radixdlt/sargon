use crate::error::Error;
use radix_engine_common::crypto::Ed25519PublicKey as EngineEd25519PublicKey;

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

impl TryFrom<&[u8]> for Ed25519PublicKey {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        EngineEd25519PublicKey::try_from(slice)
            .map(Ed25519PublicKey)
            .map_err(|_| Error::InvalidEd25519PublicKeyFromBytes)
    }
}

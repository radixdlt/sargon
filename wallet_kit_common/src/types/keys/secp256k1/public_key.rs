use radix_engine_common::crypto::Secp256k1PublicKey as EngineSecp256k1PublicKey;

pub struct Secp256k1PublicKey(EngineSecp256k1PublicKey);

impl Secp256k1PublicKey {
    pub const LENGTH: usize = 33;

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    // pub fn to_hash(&self) -> Secp256k1PublicKeyHash {
    //     Secp256k1PublicKeyHash::new_from_public_key(self)
    // }
}

impl TryFrom<&[u8]> for Secp256k1PublicKey {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        // if slice.len() != Secp256k1PublicKey::LENGTH {
        //     return Err(ParseSecp256k1PublicKeyError::InvalidLength(slice.len()));
        // }

        // Ok(Secp256k1PublicKey(copy_u8_array(slice)))
        todo!();
    }
}

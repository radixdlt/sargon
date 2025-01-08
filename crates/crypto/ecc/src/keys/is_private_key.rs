use crate::prelude::*;

pub trait IsPrivateKey<P: IsPublicKey<Self::Signature>>: Sized {
    type Signature;

    fn from_bytes(slice: &[u8]) -> Result<Self>;

    fn curve() -> SLIP10Curve;

    fn public_key(&self) -> P;

    fn sign(&self, msg_hash: &Hash) -> Self::Signature;
}

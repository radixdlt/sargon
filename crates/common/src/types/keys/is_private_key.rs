use crate::prelude::*;

pub trait IsPrivateKey<P: IsPublicKey<Self::Signature>>: Sized {
    type Signature;

    fn curve() -> SLIP10Curve;

    fn public_key(&self) -> P;

    fn sign(&self, msg_hash: &Hash) -> Self::Signature;
}

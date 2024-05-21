use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner, uniffi::Enum)]
pub enum GWPublicKey {
    Secp256k1(Secp256k1PublicKey),
    Ed25519(Ed25519PublicKey),
}

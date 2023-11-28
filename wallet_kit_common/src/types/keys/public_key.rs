use super::{ed25519::public_key::Ed25519PublicKey, secp256k1::public_key::Secp256k1PublicKey};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PublicKey {
    Ed25519(Ed25519PublicKey),
    Secp256k1(Secp256k1PublicKey),
}

impl PublicKey {
    pub fn to_hex(&self) -> String {
        match self {
            PublicKey::Ed25519(key) => key.to_hex(),
            PublicKey::Secp256k1(key) => key.to_hex(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            PublicKey::Ed25519(key) => key.to_bytes(),
            PublicKey::Secp256k1(key) => key.to_bytes(),
        }
    }
}

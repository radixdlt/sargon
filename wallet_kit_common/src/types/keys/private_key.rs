use super::{
    ed25519::private_key::Ed25519PrivateKey, public_key::PublicKey,
    secp256k1::private_key::Secp256k1PrivateKey,
};

pub enum PrivateKey {
    Ed25519(Ed25519PrivateKey),
    Secp256k1(Secp256k1PrivateKey),
}

impl PrivateKey {
    pub fn public_key(&self) -> PublicKey {
        match self {
            PrivateKey::Ed25519(key) => PublicKey::Ed25519(key.public_key()),
            PrivateKey::Secp256k1(key) => PublicKey::Secp256k1(key.public_key()),
        }
    }

    pub fn to_hex(&self) -> String {
        match self {
            PrivateKey::Ed25519(key) => key.to_hex(),
            PrivateKey::Secp256k1(key) => key.to_hex(),
        }
    }
}

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
        todo!();
    }

    pub fn hex(&self) -> String {
        todo!();
    }
}

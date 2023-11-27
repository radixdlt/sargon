use radix_engine_common::crypto::PublicKey;
use transaction::signing::PrivateKey;

pub fn private_key_bytes(private_key: &PrivateKey) -> Vec<u8> {
    match private_key {
        PrivateKey::Ed25519(key) => key.to_bytes(),
        PrivateKey::Secp256k1(key) => key.to_bytes(),
    }
}

pub fn public_key_bytes(public_key: &PublicKey) -> Vec<u8> {
    match public_key {
        PublicKey::Ed25519(key) => key.to_vec(),
        PublicKey::Secp256k1(key) => key.to_vec(),
    }
}

pub fn private_key_hex(private_key: &PrivateKey) -> String {
    hex::encode(private_key_bytes(private_key))
}

pub fn public_key_hex(public_key: &PublicKey) -> String {
    hex::encode(public_key_bytes(public_key))
}

pub fn public_key_hex_from_private(private_key: &PrivateKey) -> String {
    public_key_hex(&private_key.public_key())
}

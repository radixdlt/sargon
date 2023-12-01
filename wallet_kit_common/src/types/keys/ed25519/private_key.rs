use radix_engine_common::crypto::IsHash;
use transaction::signing::ed25519::{
    Ed25519PrivateKey as EngineEd25519PrivateKey, Ed25519Signature,
};

use super::public_key::Ed25519PublicKey;
use crate::{error::Error, types::hex_32bytes::Hex32Bytes};
use std::fmt::{Debug, Formatter};

/// An Ed25519 private key used to create cryptographic signatures, using
/// EdDSA scheme.
pub struct Ed25519PrivateKey(EngineEd25519PrivateKey);

impl Ed25519PrivateKey {
    /// Generates a new `Ed25519PrivateKey` from random bytes
    /// generated by a CSRNG, note that this is typically never
    /// used by wallets, which tend to rather use a Mnemonic and
    /// derive hierarchical deterministic keys.
    pub fn generate() -> Self {
        Self::from_hex32_bytes(Hex32Bytes::generate()).expect("Should be able to generate 32 bytes")
    }

    /// Just an alias for `Self::generate()`, generating a new
    /// key from random bytes.
    pub fn new() -> Self {
        Self::generate()
    }
}

impl PartialEq for Ed25519PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Debug for Ed25519PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl Ed25519PrivateKey {
    pub fn from_engine(engine: EngineEd25519PrivateKey) -> Self {
        Self(engine)
    }

    pub fn public_key(&self) -> Ed25519PublicKey {
        Ed25519PublicKey::from_engine(self.0.public_key())
            .expect("Public Key from EC scalar multiplication should always be valid.")
    }

    pub fn sign(&self, msg_hash: &impl IsHash) -> Ed25519Signature {
        self.0.sign(msg_hash)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn from_bytes(slice: &[u8]) -> Result<Self, Error> {
        EngineEd25519PrivateKey::from_bytes(slice)
            .map_err(|_| Error::InvalidEd25519PrivateKeyFromBytes)
            .map(Self::from_engine)
    }

    pub fn from_str(hex: &str) -> Result<Self, Error> {
        Hex32Bytes::from_hex(hex)
            .map_err(|_| Error::InvalidEd25519PrivateKeyFromString)
            .and_then(|b| Self::from_bytes(&b.to_vec()))
    }

    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, Error> {
        Self::from_bytes(bytes.as_slice())
    }

    pub fn from_hex32_bytes(bytes: Hex32Bytes) -> Result<Self, Error> {
        Self::from_vec(bytes.to_vec())
    }
}

impl TryFrom<&[u8]> for Ed25519PrivateKey {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> Result<Ed25519PrivateKey, Self::Error> {
        Ed25519PrivateKey::from_bytes(slice)
    }
}

impl TryInto<Ed25519PrivateKey> for &str {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Ed25519PrivateKey, Self::Error> {
        Ed25519PrivateKey::from_str(self)
    }
}

impl Ed25519PrivateKey {
    pub fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    /// `833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42`
    ///
    /// expected public key:
    /// `ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf`
    ///
    /// https://github.com/dalek-cryptography/ed25519-dalek/blob/main/tests/ed25519.rs#L103
    pub fn placeholder_alice() -> Self {
        Self::from_str("833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42").unwrap()
    }

    /// `1498b5467a63dffa2dc9d9e069caf075d16fc33fdd4c3b01bfadae6433767d93``

    /// expected public key:
    /// `b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde`
    ///
    /// https://cryptobook.nakov.com/digital-signatures/eddsa-sign-verify-examples
    pub fn placeholder_bob() -> Self {
        Self::from_str("1498b5467a63dffa2dc9d9e069caf075d16fc33fdd4c3b01bfadae6433767d93").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use transaction::signing::ed25519::Ed25519Signature;

    use crate::{error::Error, hash::hash, types::hex_32bytes::Hex32Bytes};

    use super::Ed25519PrivateKey;

    #[test]
    fn sign_and_verify() {
        let msg = hash("Test");
        let sk: Ed25519PrivateKey =
            "0000000000000000000000000000000000000000000000000000000000000001"
                .try_into()
                .unwrap();
        let pk = sk.public_key();
        assert_eq!(
            pk.to_hex(),
            "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29"
        );
        let sig = Ed25519Signature::from_str("cf0ca64435609b85ab170da339d415bbac87d678dfd505969be20adc6b5971f4ee4b4620c602bcbc34fd347596546675099d696265f4a42a16df343da1af980e").unwrap();

        assert_eq!(sk.sign(&msg), sig);
        assert!(pk.is_valid(&sig, &msg))
    }

    #[test]
    fn bytes_roundtrip() {
        let bytes = hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
        assert_eq!(
            Ed25519PrivateKey::from_bytes(bytes.as_slice())
                .unwrap()
                .to_bytes(),
            bytes.as_slice()
        );
    }

    #[test]
    fn hex_roundtrip() {
        let hex = "0000000000000000000000000000000000000000000000000000000000000001";
        assert_eq!(Ed25519PrivateKey::from_str(hex).unwrap().to_hex(), hex);
    }

    #[test]
    fn invalid_hex() {
        assert_eq!(
            Ed25519PrivateKey::from_str("not hex"),
            Err(Error::InvalidEd25519PrivateKeyFromString)
        );
    }

    #[test]
    fn invalid_hex_too_short() {
        assert_eq!(
            Ed25519PrivateKey::from_str("dead"),
            Err(Error::InvalidEd25519PrivateKeyFromString)
        );
    }

    #[test]
    fn invalid_bytes() {
        assert_eq!(
            Ed25519PrivateKey::from_bytes(&[0u8] as &[u8]),
            Err(Error::InvalidEd25519PrivateKeyFromBytes)
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            Ed25519PrivateKey::from_str(
                "0000000000000000000000000000000000000000000000000000000000000001"
            )
            .unwrap(),
            Ed25519PrivateKey::from_str(
                "0000000000000000000000000000000000000000000000000000000000000001"
            )
            .unwrap()
        );
    }

    #[test]
    fn debug() {
        let hex = "0000000000000000000000000000000000000000000000000000000000000001";
        assert_eq!(
            format!("{:?}", Ed25519PrivateKey::from_str(hex).unwrap()),
            hex
        );
    }

    #[test]
    fn from_vec() {
        let hex = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
        assert_eq!(
            Ed25519PrivateKey::from_vec(Vec::from(hex::decode(hex).unwrap()))
                .unwrap()
                .to_hex(),
            hex
        );
    }

    #[test]
    fn from_hex32() {
        let hex = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
        assert_eq!(
            Ed25519PrivateKey::from_hex32_bytes(Hex32Bytes::from_hex(hex).unwrap())
                .unwrap()
                .to_hex(),
            hex
        );
    }
}

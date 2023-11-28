use radix_engine_common::crypto::IsHash;
use transaction::signing::ed25519::{
    Ed25519PrivateKey as EngineEd25519PrivateKey, Ed25519Signature,
};

use crate::{error::Error, types::hex_32bytes::Hex32Bytes};

use super::public_key::Ed25519PublicKey;

pub struct Ed25519PrivateKey(EngineEd25519PrivateKey);

impl Ed25519PrivateKey {
    pub const LENGTH: usize = 32;

    pub fn public_key(&self) -> Ed25519PublicKey {
        Ed25519PublicKey::from_engine(self.0.public_key())
    }

    pub fn sign(&self, msg_hash: &impl IsHash) -> Ed25519Signature {
        todo!();
        // let keypair = Keypair {
        //     secret: SecretKey::from_bytes(self.0.as_bytes()).expect("From a valid key bytes"),
        //     public: PublicKey::from(&self.0),
        // };

        // // SHA512 is used here

        // Ed25519Signature(keypair.sign(msg_hash.as_ref()).to_bytes())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn from_bytes(slice: &[u8]) -> Result<Self, Error> {
        EngineEd25519PrivateKey::from_bytes(slice)
            .map(Ed25519PrivateKey)
            .map_err(|_| Error::InvalidEd25519PrivateKeyFromBytes)
    }

    pub fn from_str(hex: &str) -> Result<Self, Error> {
        Hex32Bytes::from_hex(hex)
            .and_then(|b| Self::from_bytes(&b.to_vec()))
            .map_err(|_| Error::InvalidEd25519PrivateKeyFromString)
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::validation::verify_ed25519;
//     use radix_engine_interface::crypto::hash;
//     use sbor::rust::str::FromStr;

//     #[test]
//     fn sign_and_verify() {
//         let test_sk = "0000000000000000000000000000000000000000000000000000000000000001";
//         let test_pk = "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29";
//         let test_message_hash = hash("Test");
//         let test_signature = "cf0ca64435609b85ab170da339d415bbac87d678dfd505969be20adc6b5971f4ee4b4620c602bcbc34fd347596546675099d696265f4a42a16df343da1af980e";
//         let sk = Ed25519PrivateKey::from_bytes(&hex::decode(test_sk).unwrap()).unwrap();
//         let pk = Ed25519PublicKey::from_str(test_pk).unwrap();
//         let sig = Ed25519Signature::from_str(test_signature).unwrap();

//         assert_eq!(sk.public_key(), pk);
//         assert_eq!(sk.sign(&test_message_hash), sig);
//         assert!(verify_ed25519(&test_message_hash, &pk, &sig));
//     }
// }

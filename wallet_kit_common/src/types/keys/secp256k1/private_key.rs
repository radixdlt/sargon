use radix_engine_common::crypto::IsHash;
use transaction::signing::secp256k1::{
    Secp256k1PrivateKey as EngineSecp256k1PrivateKey, Secp256k1Signature,
};

use crate::{error::Error, types::hex_32bytes::Hex32Bytes};

use super::public_key::Secp256k1PublicKey;

pub struct Secp256k1PrivateKey(EngineSecp256k1PrivateKey);

impl Secp256k1PrivateKey {
    pub const LENGTH: usize = 32;

    pub fn public_key(&self) -> Secp256k1PublicKey {
        Secp256k1PublicKey::from_engine(self.0.public_key())
    }

    pub fn sign(&self, msg_hash: &impl IsHash) -> Secp256k1Signature {
        // let m = Message::from_slice(msg_hash.as_ref()).expect("Hash is always a valid message");
        // let signature = SECP256K1.sign_ecdsa_recoverable(&m, &self.0);
        // let (recovery_id, signature_data) = signature.serialize_compact();

        // let mut buf = [0u8; 65];
        // buf[0] = recovery_id.to_i32() as u8;
        // buf[1..].copy_from_slice(&signature_data);
        // Secp256k1Signature(buf)
        todo!();
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn from_bytes(slice: &[u8]) -> Result<Self, Error> {
        EngineSecp256k1PrivateKey::from_bytes(slice)
            .map(Secp256k1PrivateKey)
            .map_err(|_| Error::InvalidSecp256k1PrivateKeyFromBytes)
    }

    pub fn from_str(hex: &str) -> Result<Self, Error> {
        Hex32Bytes::from_hex(hex)
            .and_then(|b| Self::from_bytes(&b.to_vec()))
            .map_err(|_| Error::InvalidSecp256k1PrivateKeyFromString)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::validation::verify_secp256k1;
//     use radix_engine_interface::crypto::hash;
//     use sbor::rust::str::FromStr;

//     #[test]
//     fn sign_and_verify() {
//         let test_sk = "0000000000000000000000000000000000000000000000000000000000000001";
//         let test_pk = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
//         let test_message_hash = hash("Test");
//         let test_signature = "00eb8dcd5bb841430dd0a6f45565a1b8bdb4a204eb868832cd006f963a89a662813ab844a542fcdbfda4086a83fbbde516214113051b9c8e42a206c98d564d7122";
//         let sk = Secp256k1PrivateKey::from_bytes(&hex::decode(test_sk).unwrap()).unwrap();
//         let pk = Secp256k1PublicKey::from_str(test_pk).unwrap();
//         let sig = Secp256k1Signature::from_str(test_signature).unwrap();

//         assert_eq!(sk.public_key(), pk);
//         assert_eq!(sk.sign(&test_message_hash), sig);
//         assert!(verify_secp256k1(&test_message_hash, &pk, &sig));
//     }
// }

use itertools::Itertools;
use radix_engine_common::crypto::PublicKey;
use serde::{Deserialize, Serialize};
use slip10::*;
use transaction::signing::PrivateKey;
use wallet_kit_common::error::Error;

use crate::{
    bip32::hd_path::HDPath,
    bip39::mnemonic::{Mnemonic, Seed},
};

use super::derivation::Derivation;

/// A BIP39 Mnemonic and BIP39 passphrase - aka "25th word" tuple,
/// from which we can derive a HD Root used for derivation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MnemonicWithPassphrase {
    pub mnemonic: Mnemonic,
    pub passphrase: String,
}

// type PrivateKeyBytes = [u8; 32]

// impl MnemonicWithPassphrase {
//     pub fn to_seed(&self) -> Seed {
//         self.mnemonic.to_seed(&self.passphrase)
//     }

//     fn derive_ed25519_private_key(seed: Seed, path: HDPath) -> PrivateKeyBytes {
//         let chain = BIP32Path::from(
//             path.components().into_iter().map(|c| c.value()).collect_vec()
//         );
//         let key = derive_key_from_path(&seed, Curve::Ed25519, &chain).map(|e| e.key).expect("Should always be able to derive")
//     }

//     pub fn derive_private_key<D>(&self, derivation: D) -> PrivateKey
//     where
//         D: Derivation,
//     {
//         /*
//                    let chain = BIP32Path::from_str(chain).unwrap();
//             let key = derive_key_from_path(&seed, Curve::Ed25519, &chain).unwrap();
//             assert_eq!(&key.chain_code[..], &Vec::from_hex(chain_code).unwrap()[..]);
//             assert_eq!(&key.key[..], &Vec::from_hex(private).unwrap()[..]);
//             assert_eq!(&key.public_key()[..], &Vec::from_hex(public).unwrap()[..]);
//         */
//         let seed = self.to_seed();

//     }
// }

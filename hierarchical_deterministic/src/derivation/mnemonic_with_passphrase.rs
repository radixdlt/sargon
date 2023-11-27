use itertools::Itertools;
use serde::{Deserialize, Serialize};
use slip10::*;
use transaction::signing::{
    ed25519::Ed25519PrivateKey, secp256k1::Secp256k1PrivateKey, PrivateKey,
};
use wallet_kit_common::error::Error;

use crate::{
    bip32::hd_path::HDPath,
    bip39::mnemonic::{Mnemonic, Seed},
};

use super::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme};

/// A BIP39 Mnemonic and BIP39 passphrase - aka "25th word" tuple,
/// from which we can derive a HD Root used for derivation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MnemonicWithPassphrase {
    pub mnemonic: Mnemonic,
    pub passphrase: String,
}

impl MnemonicWithPassphrase {
    pub fn with_passphrase(mnemonic: Mnemonic, passphrase: String) -> Self {
        Self {
            mnemonic,
            passphrase,
        }
    }

    /// Instantiates a new `MnemonicWithPassphrase` with empty passphrase (no passphrase).
    pub fn new(mnemonic: Mnemonic) -> Self {
        Self {
            mnemonic,
            passphrase: "".to_string(),
        }
    }

    /// Instantiates a new `MnemonicWithPassphrase` with empty passphrase (no passphrase),
    /// from the specified BIP39 mnemonic phrase.
    pub fn from_phrase(phrase: &str) -> Result<Self, Error> {
        Mnemonic::from_phrase(phrase).map(|m| Self::new(m))
    }
}

pub type PrivateKeyBytes = [u8; 32];

impl MnemonicWithPassphrase {
    pub fn to_seed(&self) -> Seed {
        self.mnemonic.to_seed(&self.passphrase)
    }

    fn derive_ed25519_private_key(seed: &Seed, path: &HDPath) -> Ed25519PrivateKey {
        let chain = BIP32Path::from(
            path.components()
                .into_iter()
                .map(|c| c.value())
                .collect_vec(),
        );

        let bytes = derive_key_from_path(seed, Curve::Ed25519, &chain)
            .map(|e| e.key)
            .expect("Should always be able to derive");

        Ed25519PrivateKey::from_bytes(&bytes).expect("Valid Ed25519PrivateKey bytes")
    }

    fn derive_secp256k1_private_key(seed: &Seed, path: &HDPath) -> Secp256k1PrivateKey {
        let chain: bip32::DerivationPath = path
            .to_string()
            .replace("H", "'")
            .parse()
            .expect("All HDPaths are valid bip32 paths");
        let child_xprv = bip32::XPrv::derive_from_path(&seed, &chain)
            .expect("To always be able to derive a child key using a valid BIP32 path");

        let private_key_bytes: PrivateKeyBytes = child_xprv.private_key().to_bytes().into();
        Secp256k1PrivateKey::from_bytes(&private_key_bytes)
            .expect("Valid Secp256k1PrivateKey bytes")
    }

    pub fn derive_private_key<D>(&self, derivation: D) -> PrivateKey
    where
        D: Derivation,
    {
        let seed = self.to_seed();
        let path = derivation.hd_path();
        match derivation.scheme() {
            DerivationPathScheme::Cap26 => {
                let key = Self::derive_ed25519_private_key(&seed, path);
                PrivateKey::Ed25519(key)
            }
            DerivationPathScheme::Bip44Olympia => {
                let key = Self::derive_secp256k1_private_key(&seed, path);
                PrivateKey::Secp256k1(key)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        bip39::mnemonic::Mnemonic,
        bip44::bip44_like_path::BIP44LikePath,
        cap26::{cap26_path::paths::account_path::AccountPath, cap26_repr::CAP26Repr},
    };
    use radix_engine_common::crypto::PublicKey;
    use transaction::signing::PrivateKey;

    use super::MnemonicWithPassphrase;

    fn private_key_bytes(private_key: &PrivateKey) -> Vec<u8> {
        match private_key {
            PrivateKey::Ed25519(key) => key.to_bytes(),
            PrivateKey::Secp256k1(key) => key.to_bytes(),
        }
    }
    fn public_key_bytes(public_key: &PublicKey) -> Vec<u8> {
        match public_key {
            PublicKey::Ed25519(key) => key.to_vec(),
            PublicKey::Secp256k1(key) => key.to_vec(),
        }
    }

    fn private_key_hex(private_key: &PrivateKey) -> String {
        hex::encode(private_key_bytes(private_key))
    }

    fn public_key_hex(public_key: &PublicKey) -> String {
        hex::encode(public_key_bytes(public_key))
    }
    fn public_key_hex_from_private(private_key: &PrivateKey) -> String {
        public_key_hex(&private_key.public_key())
    }

    /// Test vector: https://github.com/radixdlt/babylon-wallet-ios/blob/99161cbbb11a78f36db6991e5d5c5f092678d5fa/RadixWalletTests/CryptographyTests/SLIP10Tests/TestVectors/cap26_curve25519.json#L8
    #[test]
    fn derive_a_curve25519_key_with_cap26() {
        let mwp = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(
                "equip will roof matter pink blind book anxiety banner elbow sun young",
            )
            .unwrap(),
            "".to_string(),
        );

        let private_key: transaction::prelude::PrivateKey =
            mwp.derive_private_key(AccountPath::from_str("m/44H/1022H/12H/525H/1460H/0H").unwrap());

        assert_eq!(
            "13e971fb16cb2c816d6b9f12176e9b8ab9af1831d006114d344d119ab2715506",
            private_key_hex(&private_key)
        );
        assert_eq!(
            "451152a1cef7be603205086d4ebac0a0b78fda2ff4684b9dea5ca9ef003d4e7d",
            public_key_hex_from_private(&private_key)
        );
    }

    /// Test vector: https://github.com/radixdlt/babylon-wallet-ios/blob/99161cbbb11a78f36db6991e5d5c5f092678d5fa/RadixWalletTests/CryptographyTests/SLIP10Tests/TestVectors/bip44_secp256k1.json#L288
    #[test]
    fn derive_a_secp256k1_key_with_bip44_olympia() {
        let mwp = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(
     "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van",
            )
            .unwrap(),
            "".to_string(),
        );

        let private_key: transaction::prelude::PrivateKey =
            mwp.derive_private_key(BIP44LikePath::from_str("m/44H/1022H/0H/0/5H").unwrap());

        assert_eq!(
            "111323d507d9d690836798e3ef2e5292cfd31092b75b9b59fa584ff593a3d7e4",
            private_key_hex(&private_key)
        );

        assert_eq!(
            "03e78cdb2e0b7ea6e55e121a58560ccf841a913d3a4a9b8349e0ef00c2102f48d8",
            public_key_hex_from_private(&private_key)
        );
    }
}

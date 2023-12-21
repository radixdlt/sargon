use super::hierarchical_deterministic_private_key::HierarchicalDeterministicPrivateKey;
use derive_getters::Getters;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use wallet_kit_common::types::keys::{
    ed25519::private_key::Ed25519PrivateKey, secp256k1::private_key::Secp256k1PrivateKey,
    slip10_curve::SLIP10Curve,
};

use super::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme};
use crate::{
    bip32::HDPath,
    bip39::mnemonic::{Mnemonic, Seed},
};
use wallet_kit_common::error::hdpath_error::HDPathError as Error;

/// A BIP39 Mnemonic and BIP39 passphrase - aka "25th word" tuple,
/// from which we can derive a HD Root used for derivation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Getters)]
#[serde(rename_all = "camelCase")]
pub struct MnemonicWithPassphrase {
    mnemonic: Mnemonic,
    passphrase: String,
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

#[cfg(any(test, feature = "placeholder"))]
impl MnemonicWithPassphrase {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::with_passphrase(Mnemonic::placeholder(), "radix".to_string())
    }
}

pub type PrivateKeyBytes = [u8; 32];

impl MnemonicWithPassphrase {
    pub fn to_seed(&self) -> Seed {
        self.mnemonic.to_seed(&self.passphrase)
    }

    fn derive_ed25519_private_key(seed: &Seed, path: &HDPath) -> Ed25519PrivateKey {
        let chain = slip10::BIP32Path::from(
            path.components()
                .into_iter()
                .map(|c| c.value())
                .collect_vec(),
        );

        let bytes = slip10::derive_key_from_path(seed, slip10::Curve::Ed25519, &chain)
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

    #[cfg(not(tarpaulin_include))] // false negative
    pub fn derive_private_key<D>(&self, derivation: D) -> HierarchicalDeterministicPrivateKey
    where
        D: Derivation,
    {
        let seed = self.to_seed();
        let path = derivation.derivation_path();
        match derivation.scheme() {
            DerivationPathScheme::Cap26 => {
                assert_eq!(derivation.scheme().curve(), SLIP10Curve::Curve25519);
                let key = Self::derive_ed25519_private_key(&seed, path.hd_path());
                HierarchicalDeterministicPrivateKey::new(key.into(), path)
            }
            DerivationPathScheme::Bip44Olympia => {
                assert_eq!(derivation.scheme().curve(), SLIP10Curve::Secp256k1);
                let key = Self::derive_secp256k1_private_key(&seed, path.hd_path());
                HierarchicalDeterministicPrivateKey::new(key.into(), path)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        bip39::mnemonic::Mnemonic,
        bip44::bip44_like_path::BIP44LikePath,
        cap26::{
            cap26_key_kind::CAP26KeyKind, cap26_path::paths::account_path::AccountPath,
            cap26_repr::CAP26Repr,
        },
        derivation::derivation::Derivation,
    };
    use wallet_kit_common::{json::assert_eq_after_json_roundtrip, network_id::NetworkID};

    use super::MnemonicWithPassphrase;

    #[test]
    fn with_passphrase() {
        let phrase = "equip will roof matter pink blind book anxiety banner elbow sun young";
        let passphrase = "25th";
        let mwp = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(phrase).unwrap(),
            passphrase.to_string(),
        );
        assert_eq!(mwp.mnemonic().phrase(), phrase);
        assert_eq!(mwp.passphrase(), passphrase);
    }

    #[test]
    fn new_eq_from_phrase() {
        let phrase = "equip will roof matter pink blind book anxiety banner elbow sun young";
        assert_eq!(
            MnemonicWithPassphrase::new(Mnemonic::from_phrase(phrase).unwrap()),
            MnemonicWithPassphrase::from_phrase(phrase).unwrap()
        );
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

        let private_key =
            mwp.derive_private_key(AccountPath::from_str("m/44H/1022H/12H/525H/1460H/0H").unwrap());

        assert_eq!(
            "13e971fb16cb2c816d6b9f12176e9b8ab9af1831d006114d344d119ab2715506",
            private_key.to_hex()
        );
        assert_eq!(
            "451152a1cef7be603205086d4ebac0a0b78fda2ff4684b9dea5ca9ef003d4e7d",
            private_key.public_key().to_hex()
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

        let private_key =
            mwp.derive_private_key(BIP44LikePath::from_str("m/44H/1022H/0H/0/5H").unwrap());

        assert_eq!(
            "111323d507d9d690836798e3ef2e5292cfd31092b75b9b59fa584ff593a3d7e4",
            private_key.to_hex()
        );

        assert_eq!(
            "03e78cdb2e0b7ea6e55e121a58560ccf841a913d3a4a9b8349e0ef00c2102f48d8",
            private_key.public_key().to_hex()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(
     "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van",
            )
            .unwrap(),
            "25th".to_string(),
        );

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "mnemonic": "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van",
                "passphrase": "25th"
            }
            "#,
        );
    }

    #[test]
    fn keys_for_placeholder() {
        let mwp = MnemonicWithPassphrase::placeholder();
        let path = AccountPath::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, 0);
        let private_key = mwp.derive_private_key(path.clone());

        assert_eq!(path.to_string(), "m/44H/1022H/1H/525H/1460H/0H");

        assert_eq!(
            "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003",
            private_key.to_hex()
        );
        assert_eq!(
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b",
            private_key.public_key().to_hex()
        );
    }
}

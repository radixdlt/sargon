use crate::PublicKey;
use serde::{Deserialize, Serialize};

use crate::DerivationPath;

#[cfg(any(test, feature = "placeholder"))]
use crate::{
    AccountPath, BIP44LikePath, CAP26KeyKind, CAP26Repr, Derivation, Mnemonic,
    MnemonicWithPassphrase,
};

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;
#[cfg(any(test, feature = "placeholder"))]
use crate::NetworkID;

/// The **source** of a virtual hierarchical deterministic badge, contains a
/// derivation path and public key, from which a private key is derived which
/// produces virtual badges (signatures).
///
/// The `.device` `FactorSource` produces `FactorInstance`s with this kind if badge source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct HierarchicalDeterministicPublicKey {
    /// The expected public key of the private key derived at `derivationPath`
    pub public_key: PublicKey,

    /// The HD derivation path for the key pair which produces virtual badges (signatures).
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicPublicKey {
    pub fn new(public_key: PublicKey, derivation_path: DerivationPath) -> Self {
        Self {
            public_key,
            derivation_path,
        }
    }
}

impl HierarchicalDeterministicPublicKey {
    pub fn to_hex(&self) -> String {
        self.public_key.to_hex()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.public_key.to_bytes()
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for HierarchicalDeterministicPublicKey {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        let mwp = MnemonicWithPassphrase::placeholder();
        let path = AccountPath::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, 0);
        let private_key = mwp.derive_private_key(path.clone());

        assert_eq!(path.to_string(), "m/44H/1022H/1H/525H/1460H/0H");

        assert_eq!(
            "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003",
            private_key.to_hex()
        );
        let public_key = private_key.public_key();
        assert_eq!(
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b",
            public_key.to_hex()
        );

        public_key
    }

    fn placeholder_other() -> Self {
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
        let public_key = private_key.public_key();
        assert_eq!(
            "03e78cdb2e0b7ea6e55e121a58560ccf841a913d3a4a9b8349e0ef00c2102f48d8",
            public_key.to_hex()
        );

        public_key
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::HierarchicalDeterministicPublicKey;

    #[test]
    fn to_hex() {
        assert_eq!(
            HierarchicalDeterministicPublicKey::placeholder().to_hex(),
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
        );
    }

    #[test]
    fn json() {
        let model = HierarchicalDeterministicPublicKey::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"publicKey": {
					"curve": "curve25519",
					"compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
				},
				"derivationPath": {
					"scheme": "cap26",
					"path": "m/44H/1022H/1H/525H/1460H/0H"
				}
			}
       "#,
        );
    }
}

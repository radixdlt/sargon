use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use wallet_kit_common::{network_id::NetworkID, types::keys::public_key::PublicKey};

use crate::{
    cap26::{
        cap26_key_kind::CAP26KeyKind, cap26_path::paths::account_path::AccountPath,
        cap26_repr::CAP26Repr,
    },
    derivation::{derivation::Derivation, derivation_path::DerivationPath},
};

use super::mnemonic_with_passphrase::MnemonicWithPassphrase;

/// The **source** of a virtual hierarchical deterministic badge, contains a
/// derivation path and public key, from which a private key is derived which
/// produces virtual badges (signatures).
///
/// The `.device` `FactorSource` produces `FactorInstance`s with this kind if badge source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Getters)]
#[serde(rename_all = "camelCase")]
pub struct HierarchicalDeterministicPublicKey {
    /// The expected public key of the private key derived at `derivationPath`
    public_key: PublicKey,

    /// The HD derivation path for the key pair which produces virtual badges (signatures).
    derivation_path: DerivationPath,
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
impl HierarchicalDeterministicPublicKey {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
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
        // Self::new(public_key, path.into())
        return public_key;
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

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

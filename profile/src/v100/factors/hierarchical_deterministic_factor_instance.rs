use hierarchical_deterministic::derivation::derivation_path::DerivationPath;
use radix_engine_common::crypto::PublicKey;
use serde::{Deserialize, Serialize};
use transaction::signing::ed25519::Ed25519PrivateKey;

use super::factor_source_id_from_hash::FactorSourceIDFromHash;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HierarchicalDeterministicFactorInstance {
    #[serde(rename = "factorSourceID")]
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: PublicKey,
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicFactorInstance {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        public_key: PublicKey,
        derivation_path: DerivationPath,
    ) -> Self {
        Self {
            factor_source_id,
            public_key,
            derivation_path,
        }
    }
}

impl HierarchicalDeterministicFactorInstance {
    pub fn placeholder() -> Self {
        let private_key = Ed25519PrivateKey::from_u64(1337).unwrap();
        let public_key = private_key.public_key();
        Self::new(
            FactorSourceIDFromHash::placeholder(),
            PublicKey::Ed25519(public_key),
            DerivationPath::placeholder(),
        )
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::HierarchicalDeterministicFactorInstance;

    //#[test]
    fn json_roundtrip() {
        let model = HierarchicalDeterministicFactorInstance::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"badge": {
					"virtualSource": {
						"hierarchicalDeterministicPublicKey": {
							"publicKey": {
								"curve": "curve25519",
								"compressedData": "3feb8194ead2e526fbcc4c1673a7a8b29d8cee0b32bb9393692f739821dd256b"
							},
							"derivationPath": {
								"scheme": "cap26",
								"path": "m/44H/1022H/14H/525H/1460H/0H"
							}
						},
						"discriminator": "hierarchicalDeterministicPublicKey"
					},
					"discriminator": "virtualSource"
				},
				"factorSourceID": {
					"fromHash": {
						"kind": "device",
						"body": "c9e67a9028fb3150304c77992710c35c8e479d4fa59f7c45a96ce17f6fdf1d2c"
					},
					"discriminator": "fromHash"
				}
			}
            "#,
        );
    }
}

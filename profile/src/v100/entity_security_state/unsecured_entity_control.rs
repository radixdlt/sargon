use serde::{Deserialize, Serialize};

use crate::v100::factors::hierarchical_deterministic_factor_instance::HierarchicalDeterministicFactorInstance;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UnsecuredEntityControl {
    // /// The factor instance which was used to create this unsecured entity, which
    // /// also controls this entity and is used for signing transactions.
    pub transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The factor instance which can be used for ROLA.
    pub authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
}

impl UnsecuredEntityControl {
    pub fn with_authentication_signing(
        transaction_signing: HierarchicalDeterministicFactorInstance,
        authentication_signing: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self {
            transaction_signing,
            authentication_signing: Some(authentication_signing),
        }
    }
    pub fn new(transaction_signing: HierarchicalDeterministicFactorInstance) -> Self {
        Self {
            transaction_signing,
            authentication_signing: Option::None,
        }
    }
}

impl UnsecuredEntityControl {
    pub fn placeholder() -> Self {
        Self::new(HierarchicalDeterministicFactorInstance::placeholder())
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::UnsecuredEntityControl;
    // #[test]
    fn json_roundtrip() {
        let model = UnsecuredEntityControl::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"transactionSigning": {
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
			}
            "#,
        );
    }
}

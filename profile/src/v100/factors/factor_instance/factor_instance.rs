use crate::HierarchicalDeterministicPublicKey;
use serde::{Deserialize, Serialize};

use super::{
    badge_virtual_source::FactorInstanceBadgeVirtualSource,
    factor_instance_badge::FactorInstanceBadge,
};
use crate::v100::factors::factor_source_id::FactorSourceID;

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct FactorInstance {
    /// The ID of the `FactorSource` that was used to produce this
    /// factor instance. We will lookup the `FactorSource` in the
    /// `Profile` and can present user with instruction to re-access
    /// this factor source in order control the `badge`.
    #[serde(rename = "factorSourceID")]
    pub factor_source_id: FactorSourceID,

    /// Either a "physical" badge (NFT) or some source for recreation of a producer
    /// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
    /// is derived which produces virtual badges (signatures).
    pub badge: FactorInstanceBadge,
}

impl FactorInstance {
    pub fn new(factor_source_id: FactorSourceID, badge: FactorInstanceBadge) -> Self {
        Self {
            factor_source_id,
            badge,
        }
    }

    pub fn with_hierarchical_deterministic_public_key(
        factor_source_id: FactorSourceID,
        hierarchical_deterministic_public_key: HierarchicalDeterministicPublicKey,
    ) -> Self {
        Self::new(
            factor_source_id,
            FactorInstanceBadge::Virtual {
                value: FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                    value: hierarchical_deterministic_public_key,
                },
            },
        )
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for FactorInstance {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(
            FactorSourceID::placeholder(),
            FactorInstanceBadge::placeholder(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new(
            FactorSourceID::placeholder_other(),
            FactorInstanceBadge::placeholder_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::FactorInstance;

    #[test]
    fn equality() {
        assert_eq!(FactorInstance::placeholder(), FactorInstance::placeholder());
        assert_eq!(
            FactorInstance::placeholder_other(),
            FactorInstance::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorInstance::placeholder(),
            FactorInstance::placeholder_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = FactorInstance::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"badge": {
					"virtualSource": {
						"hierarchicalDeterministicPublicKey": {
							"publicKey": {
								"curve": "curve25519",
								"compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
							},
							"derivationPath": {
								"scheme": "cap26",
								"path": "m/44H/1022H/1H/525H/1460H/0H"
							}
						},
						"discriminator": "hierarchicalDeterministicPublicKey"
					},
					"discriminator": "virtualSource"
				},
				"factorSourceID": {
					"fromHash": {
						"kind": "device",
						"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
					},
					"discriminator": "fromHash"
				}
			}
            "#,
        );
    }
}

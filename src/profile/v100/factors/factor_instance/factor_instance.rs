use crate::prelude::*;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
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
    pub fn new(
        factor_source_id: FactorSourceID,
        badge: FactorInstanceBadge,
    ) -> Self {
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

impl HasSampleValues for FactorInstance {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(FactorSourceID::sample(), FactorInstanceBadge::sample())
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::new(
            FactorSourceID::sample_other(),
            FactorInstanceBadge::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(FactorInstance::sample(), FactorInstance::sample());
        assert_eq!(
            FactorInstance::sample_other(),
            FactorInstance::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(FactorInstance::sample(), FactorInstance::sample_other());
    }

    #[test]
    fn json_roundtrip() {
        let model = FactorInstance::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"badge": {
					"virtualSource": {
						"hierarchicalDeterministicPublicKey": {
							"publicKey": {
								"curve": "curve25519",
								"compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
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
						"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
					},
					"discriminator": "fromHash"
				}
			}
            "#,
        );
    }
}

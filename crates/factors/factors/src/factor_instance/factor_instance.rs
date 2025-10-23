use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
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

// TODO: It looks like we wont need the `FactorInstanceBadge` type, i.e. that we wont
// have to handle the scenario of some Instances being physical - not HD, so when
// we have flattened this `FactorInstance` to only work with `HierarchicalDeterministicFactorInstance`
// we can update this implementation to be `IsKeySpaceAware` instead of `IsMaybeKeySpaceAware`.
impl IsMaybeKeySpaceAware for FactorInstance {
    fn maybe_key_space(&self) -> Option<KeySpace> {
        match self.badge.clone() {
            FactorInstanceBadge::Virtual { value } => Some(value.key_space()),
            // FactorInstanceBadge::Physical { .. } => None,
        }
    }
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

    pub fn try_as_hd_factor_instances(
        &self,
    ) -> Result<HierarchicalDeterministicFactorInstance> {
        HierarchicalDeterministicFactorInstance::try_from_factor_instance(
            self.clone(),
        )
    }
}

impl From<HierarchicalDeterministicFactorInstance> for FactorInstance {
    fn from(value: HierarchicalDeterministicFactorInstance) -> Self {
        Self::with_hierarchical_deterministic_public_key(
            value.factor_source_id.into(),
            value.hd_public_key(),
        )
    }
}

impl FactorInstance {
    pub fn sample_mainnet_account_securified_fs0_idx_0() -> Self {
        HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0).into()
    }
    pub fn sample_mainnet_account_securified_fs0_idx_1() -> Self {
        HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(1).into()
    }
    pub fn sample_mainnet_account_securified_fs0_idx_2() -> Self {
        HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(2).into()
    }
    pub fn sample_mainnet_account_securified_fs1_idx_0() -> Self {
        HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(0).into()
    }
    pub fn sample_mainnet_account_securified_fs1_idx_1() -> Self {
        HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(1).into()
    }
    pub fn sample_mainnet_account_securified_fs1_idx_2() -> Self {
        HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(2).into()
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

#[cfg(debug_assertions)]
impl FactorInstance {
    pub fn sign_hash(&self, hash: &Hash) -> SignatureWithPublicKey {
        let mwp = self
            .factor_source_id
            .into_hash()
            .unwrap()
            .sample_associated_mnemonic();
        mwp.sign(
            hash,
            &self.try_as_hd_factor_instances().unwrap().derivation_path(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorInstance;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_securified() {
        let suts = vec![
            SUT::sample_mainnet_account_securified_fs0_idx_0(),
            SUT::sample_mainnet_account_securified_fs0_idx_1(),
            SUT::sample_mainnet_account_securified_fs0_idx_2(),
            SUT::sample_mainnet_account_securified_fs1_idx_0(),
            SUT::sample_mainnet_account_securified_fs1_idx_1(),
            SUT::sample_mainnet_account_securified_fs1_idx_2(),
        ];
        assert_eq!(HashSet::<SUT>::from_iter(suts.clone()).len(), 6);
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
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

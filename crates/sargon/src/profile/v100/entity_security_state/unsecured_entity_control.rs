use crate::prelude::*;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UnsecuredEntityControl {
    /// The factor instance which was used to create this unsecured entity, which
    /// also controls this entity and is used for signing transactions.
    pub transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The provisional security structure configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional: Option<ProvisionalSecurifiedConfig>,
}

impl HasProvisionalSecurifiedConfig for UnsecuredEntityControl {
    fn get_provisional(&self) -> Option<ProvisionalSecurifiedConfig> {
        self.provisional.clone()
    }

    fn set_provisional_unchecked(
        &mut self,
        provisional: impl Into<Option<ProvisionalSecurifiedConfig>>,
    ) {
        self.provisional = provisional.into();
    }
}

impl HasFactorInstances for UnsecuredEntityControl {
    fn unique_factor_instances(&self) -> IndexSet<FactorInstance> {
        IndexSet::just(self.transaction_signing.factor_instance())
    }
}

impl UnsecuredEntityControl {
    pub fn with_entity_creating_factor_instance<T>(
        entity_creating_factor_instance: HDFactorInstanceTransactionSigning<T>,
    ) -> Self
    where
        T: IsEntityPath,
    {
        Self {
            transaction_signing: entity_creating_factor_instance.into(),
            provisional: None,
        }
    }

    #[cfg(not(tarpaulin_include))] // false negative
    pub fn new(
        transaction_signing: HierarchicalDeterministicFactorInstance,
        provisional: impl Into<Option<ProvisionalSecurifiedConfig>>,
    ) -> Result<Self> {
        let key_kind = transaction_signing.get_key_kind();
        if key_kind != CAP26KeyKind::TransactionSigning {
            return Err(
                CommonError::WrongKeyKindOfTransactionSigningFactorInstance,
            );
        }
        Ok(Self {
            transaction_signing,
            provisional: provisional.into(),
        })
    }

    pub fn with_transaction_signing_only(
        transaction_signing: HierarchicalDeterministicFactorInstance,
    ) -> Result<Self> {
        Self::new(transaction_signing, None)
    }
}

impl HasSampleValues for UnsecuredEntityControl {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::with_transaction_signing_only(
            HierarchicalDeterministicFactorInstance::sample(),
        )
        .expect("Valid sample")
    }

    fn sample_other() -> Self {
        Self::with_transaction_signing_only(
            HierarchicalDeterministicFactorInstance::sample_other(),
        )
        .expect("Valid sample")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UnsecuredEntityControl;

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
    fn json_roundtrip() {
        let model = SUT::sample();
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
			}
            "#,
        );
    }
}

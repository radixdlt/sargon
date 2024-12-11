use crate::prelude::*;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct UnsecuredEntityControl {
    // /// The factor instance which was used to create this unsecured entity, which
    // /// also controls this entity and is used for signing transactions.
    pub transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The factor instance which can be used for ROLA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
}

impl HasFactorInstances for UnsecuredEntityControl {
    fn unique_factor_instances(&self) -> IndexSet<FactorInstance> {
        let mut set = IndexSet::new();
        set.insert(self.transaction_signing.factor_instance());
        if let Some(authentication_signing) =
            self.authentication_signing.as_ref()
        {
            set.insert(authentication_signing.factor_instance());
        }
        set
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
            authentication_signing: None,
        }
    }

    #[cfg(not(tarpaulin_include))] // false negative
    pub fn new(
        transaction_signing: HierarchicalDeterministicFactorInstance,
        authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
    ) -> Result<Self> {
        let is_invalid_auth_signing_key = authentication_signing
            .as_ref()
            .map(|auth| {
                auth.get_key_kind() != CAP26KeyKind::AuthenticationSigning
            })
            .unwrap_or(false);

        if is_invalid_auth_signing_key {
            return Err(
                CommonError::WrongKeyKindOfAuthenticationSigningFactorInstance,
            );
        }

        let key_kind = transaction_signing.get_key_kind();
        if key_kind != CAP26KeyKind::TransactionSigning {
            return Err(
                CommonError::WrongKeyKindOfTransactionSigningFactorInstance,
            );
        }
        Ok(Self {
            transaction_signing,
            authentication_signing,
        })
    }

    pub fn with_transaction_signing_only(
        transaction_signing: HierarchicalDeterministicFactorInstance,
    ) -> Result<Self> {
        Self::new(transaction_signing, None)
    }
}

impl UnsecuredEntityControl {
    /// Returns whether the entity is controlled by the given factor source.
    pub fn is_controlled_by_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> bool {
        match factor_source.factor_source_id() {
            // TODO: This is what it was currently done on iOS, confirm if it is correct to ignore authentication_signing
            FactorSourceID::Hash { value } => {
                value == self.transaction_signing.factor_source_id
            }
            FactorSourceID::Address { .. } => false,
        }
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
    fn with_auth_signing() {
        let tx_sign = HierarchicalDeterministicFactorInstance::sample();
        let auth_sign =
            HierarchicalDeterministicFactorInstance::sample_auth_signing();
        let control = SUT::new(tx_sign, Some(auth_sign.clone())).unwrap();
        assert_eq!(control.authentication_signing, Some(auth_sign));
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

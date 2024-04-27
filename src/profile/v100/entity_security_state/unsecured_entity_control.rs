use crate::prelude::*;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct UnsecuredEntityControl {
    // /// The factor instance which was used to create this unsecured entity, which
    // /// also controls this entity and is used for signing transactions.
    pub transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The factor instance which can be used for ROLA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
}

impl UnsecuredEntityControl {
    pub fn with_entity_creating_factor_instance<T>(
        entity_creating_factor_instance: HDFactorInstanceTransactionSigning<T>,
    ) -> Self
    where
        T: IsEntityPath + Clone,
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
            .and_then(|auth| {
                auth.key_kind().map(|key_kind| {
                    key_kind != CAP26KeyKind::AuthenticationSigning
                })
            })
            .unwrap_or(false);

        if is_invalid_auth_signing_key {
            return Err(
                CommonError::WrongKeyKindOfAuthenticationSigningFactorInstance,
            );
        }

        if let Some(key_kind) = transaction_signing.key_kind() {
            if key_kind != CAP26KeyKind::TransactionSigning {
                return Err(
                    CommonError::WrongKeyKindOfTransactionSigningFactorInstance,
                );
            }
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
			}
            "#,
        );
    }
}

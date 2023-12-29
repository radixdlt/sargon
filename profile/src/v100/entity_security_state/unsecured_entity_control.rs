use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::v100::{HDFactorInstanceAccountCreation, HierarchicalDeterministicFactorInstance};
use crate::CommonError as Error;

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct UnsecuredEntityControl {
    // /// The factor instance which was used to create this unsecured entity, which
    // /// also controls this entity and is used for signing transactions.
    transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The factor instance which can be used for ROLA.
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
}

impl UnsecuredEntityControl {
    pub fn with_account_creating_factor_instance(
        account_creating_factor_instance: HDFactorInstanceAccountCreation,
    ) -> Self {
        Self {
            transaction_signing: account_creating_factor_instance.into(),
            authentication_signing: None,
        }
    }

    #[cfg(not(tarpaulin_include))] // false negative
    pub fn new(
        transaction_signing: HierarchicalDeterministicFactorInstance,
        authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
    ) -> Result<Self, Error> {
        use crate::CAP26KeyKind;

        if let Some(auth) = &authentication_signing {
            if let Some(key_kind) = auth.key_kind() {
                if key_kind != CAP26KeyKind::AuthenticationSigning {
                    return Err(Error::WrongKeyKindOfAuthenticationSigningFactorInstance);
                }
            }
        }
        if let Some(key_kind) = transaction_signing.key_kind() {
            if key_kind != CAP26KeyKind::TransactionSigning {
                return Err(Error::WrongKeyKindOfTransactionSigningFactorInstance);
            }
        }
        Ok(Self {
            transaction_signing,
            authentication_signing,
        })
    }

    pub fn with_transaction_signing_only(
        transaction_signing: HierarchicalDeterministicFactorInstance,
    ) -> Result<Self, Error> {
        Self::new(transaction_signing, None)
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for UnsecuredEntityControl {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::with_transaction_signing_only(HierarchicalDeterministicFactorInstance::placeholder())
            .expect("Valid placeholder")
    }

    fn placeholder_other() -> Self {
        Self::with_transaction_signing_only(
            HierarchicalDeterministicFactorInstance::placeholder_other(),
        )
        .expect("Valid placeholder")
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use crate::v100::HierarchicalDeterministicFactorInstance;

    use super::UnsecuredEntityControl;

    #[test]
    fn equality() {
        assert_eq!(
            UnsecuredEntityControl::placeholder(),
            UnsecuredEntityControl::placeholder()
        );
        assert_eq!(
            UnsecuredEntityControl::placeholder_other(),
            UnsecuredEntityControl::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            UnsecuredEntityControl::placeholder(),
            UnsecuredEntityControl::placeholder_other()
        );
    }

    #[test]
    fn with_auth_signing() {
        let tx_sign = HierarchicalDeterministicFactorInstance::placeholder();
        let auth_sign = HierarchicalDeterministicFactorInstance::placeholder_auth_signing();
        let control = UnsecuredEntityControl::new(tx_sign, Some(auth_sign.clone())).unwrap();
        assert_eq!(control.authentication_signing, Some(auth_sign));
    }

    #[test]
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

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use super::unsecured_entity_control::UnsecuredEntityControl;

#[cfg(any(test, feature = "placeholder"))]
use wallet_kit_common::HasPlaceholder;

/// Describes the state an entity - Account or Persona - is in in regards to how
/// the user controls it, i.e. if it is controlled by a single factor (private key)
///  or an `AccessController` with a potential Multi-Factor setup.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(remote = "Self")]
pub enum EntitySecurityState {
    /// The account is controlled by a single factor (private key)
    #[serde(rename = "unsecuredEntityControl")]
    Unsecured(UnsecuredEntityControl),
}

impl<'de> Deserialize<'de> for EntitySecurityState {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "discriminator")]
            _ignore: String,
            #[serde(flatten, with = "EntitySecurityState")]
            inner: EntitySecurityState,
        }
        Wrapper::deserialize(deserializer).map(|w| w.inner)
    }
}

impl Serialize for EntitySecurityState {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EntitySecurityState", 2)?;
        match self {
            EntitySecurityState::Unsecured(control) => {
                state.serialize_field("discriminator", "unsecured")?;
                state.serialize_field("unsecuredEntityControl", control)?;
            }
        }
        state.end()
    }
}

impl From<UnsecuredEntityControl> for EntitySecurityState {
    fn from(value: UnsecuredEntityControl) -> Self {
        Self::Unsecured(value)
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for EntitySecurityState {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::Unsecured(UnsecuredEntityControl::placeholder())
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::Unsecured(UnsecuredEntityControl::placeholder_other())
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::EntitySecurityState;

    #[test]
    fn equality() {
        assert_eq!(
            EntitySecurityState::placeholder(),
            EntitySecurityState::placeholder()
        );
        assert_eq!(
            EntitySecurityState::placeholder_other(),
            EntitySecurityState::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            EntitySecurityState::placeholder(),
            EntitySecurityState::placeholder_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = EntitySecurityState::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"unsecuredEntityControl": {
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
				},
				"discriminator": "unsecured"
			}
            "#,
        );
    }
}

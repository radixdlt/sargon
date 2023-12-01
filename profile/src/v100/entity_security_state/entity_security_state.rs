use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use super::unsecured_entity_control::UnsecuredEntityControl;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(remote = "Self")]
pub enum EntitySecurityState {
    #[serde(rename = "unsecuredEntityControl")]
    Unsecured(UnsecuredEntityControl),
}

impl<'de> Deserialize<'de> for EntitySecurityState {
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

impl EntitySecurityState {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::Unsecured(UnsecuredEntityControl::placeholder())
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::EntitySecurityState;

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

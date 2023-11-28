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
                let discriminant = "unsecuredEntityControl";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, control)?;
            }
        }
        state.end()
    }
}

impl EntitySecurityState {
    pub fn placeholder() -> Self {
        Self::Unsecured(UnsecuredEntityControl::placeholder())
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn json_roundtrip() {
    //     // let model = Account::with_values(
    //     //     "account_tdx_e_128vkt2fur65p4hqhulfv3h0cknrppwtjsstlttkfamj4jnnpm82gsw"
    //     //         .try_into()
    //     //         .unwrap(),
    //     //     "Zaba 0".try_into().unwrap(),
    //     //     0.try_into().unwrap(),
    //     // );
    //     assert_eq_after_json_roundtrip(
    //         &model,
    //         r#"
    //         {
    // 			"unsecuredEntityControl": {
    // 				"transactionSigning": {
    // 					"badge": {
    // 						"virtualSource": {
    // 							"hierarchicalDeterministicPublicKey": {
    // 								"publicKey": {
    // 									"curve": "curve25519",
    // 									"compressedData": "3feb8194ead2e526fbcc4c1673a7a8b29d8cee0b32bb9393692f739821dd256b"
    // 								},
    // 								"derivationPath": {
    // 									"scheme": "cap26",
    // 									"path": "m/44H/1022H/14H/525H/1460H/0H"
    // 								}
    // 							},
    // 							"discriminator": "hierarchicalDeterministicPublicKey"
    // 						},
    // 						"discriminator": "virtualSource"
    // 					},
    // 					"factorSourceID": {
    // 						"fromHash": {
    // 							"kind": "device",
    // 							"body": "c9e67a9028fb3150304c77992710c35c8e479d4fa59f7c45a96ce17f6fdf1d2c"
    // 						},
    // 						"discriminator": "fromHash"
    // 					}
    // 				}
    // 			},
    // 			"discriminator": "unsecured"
    // 		}
    //         "#,
    //     );
    // }
}

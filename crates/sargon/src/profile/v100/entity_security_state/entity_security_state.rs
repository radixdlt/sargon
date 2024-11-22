use crate::prelude::*;

/// Describes the state an entity - Account or Persona - is in, in regards to how
/// the user controls it, i.e. if it is controlled by a single factor (private key)
///  or an `AccessController` with a potential Multi-Factor setup.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumAsInner,
)]
#[serde(untagged, remote = "Self")]
pub enum EntitySecurityState {
    /// The account is controlled by a single factor (private key)
    Unsecured {
        #[serde(rename = "unsecuredEntityControl")]
        value: UnsecuredEntityControl,
    },
    /// The account is controlled by multi-factor
    Securified {
        #[serde(rename = "securedEntityControl")]
        value: SecuredEntityControl,
    },
}

impl<'de> Deserialize<'de> for EntitySecurityState {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(flatten, with = "EntitySecurityState")]
            value: EntitySecurityState,
        }
        Wrapper::deserialize(deserializer).map(|w| w.value)
    }
}

impl Serialize for EntitySecurityState {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state =
            serializer.serialize_struct("EntitySecurityState", 2)?;
        match self {
            EntitySecurityState::Unsecured { value } => {
                state.serialize_field("discriminator", "unsecured")?;
                state.serialize_field("unsecuredEntityControl", value)?;
            }
            EntitySecurityState::Securified { value } => {
                state.serialize_field("discriminator", "securified")?;
                state.serialize_field("securedEntityControl", value)?;
            }
        }
        state.end()
    }
}

impl From<UnsecuredEntityControl> for EntitySecurityState {
    fn from(value: UnsecuredEntityControl) -> Self {
        Self::Unsecured { value }
    }
}

impl From<SecuredEntityControl> for EntitySecurityState {
    fn from(value: SecuredEntityControl) -> Self {
        Self::Securified { value }
    }
}

impl HasSampleValues for EntitySecurityState {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::Unsecured {
            value: UnsecuredEntityControl::sample(),
        }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::Unsecured {
            value: UnsecuredEntityControl::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntitySecurityState;

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
    fn json_roundtrip_unsecurified() {
        let model = SUT::sample();

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
                },
                "discriminator": "unsecured"
            }
            "#,
        );
    }

    #[test]
    fn json_roundtrip_securified() {
        let secured_entity_control = SecuredEntityControl::sample();

        let model = EntitySecurityState::Securified {
            value: secured_entity_control,
        };

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
              "discriminator": "securified",
              "securedEntityControl": {
                "veci": null,
                "accessControllerAddress": "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a",
                "securityStructure": {
                  "securityStructureId": "ffffffff-ffff-ffff-ffff-ffffffffffff",
                  "matrixOfFactors": {
                    "primaryRole": {
                      "thresholdFactors": [
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "device",
                              "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "427969814e15d74c3ff4d9971465cb709d210c8a7627af9466bdaa67bd0929b7"
                                },
                                "derivationPath": {
                                  "scheme": "cap26",
                                  "path": "m/44H/1022H/1H/525H/1460H/0S"
                                }
                              }
                            }
                          }
                        }
                      ],
                      "threshold": 1,
                      "overrideFactors": [
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "device",
                              "body": "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "e0293d4979bc303ea4fe361a62baf9c060c7d90267972b05c61eead9ef3eed3e"
                                },
                                "derivationPath": {
                                  "scheme": "cap26",
                                  "path": "m/44H/1022H/1H/525H/1460H/0S"
                                }
                              }
                            }
                          }
                        }
                      ]
                    },
                    "recoveryRole": {
                      "thresholdFactors": [
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "device",
                              "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "161a65a7b4f374d81bf5e7f73669f5b09b684a860812ec1a34f3220b6ffe8dcf"
                                },
                                "derivationPath": {
                                  "scheme": "cap26",
                                  "path": "m/44H/1022H/1H/525H/1460H/54S"
                                }
                              }
                            }
                          }
                        }
                      ],
                      "threshold": 1,
                      "overrideFactors": [
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "device",
                              "body": "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "23fa85f95c79684d2768f46ec4379b5e031757b727f76cfd01a50bd4cf8afcff"
                                },
                                "derivationPath": {
                                  "scheme": "cap26",
                                  "path": "m/44H/1022H/1H/525H/1460H/237S"
                                }
                              }
                            }
                          }
                        }
                      ]
                    },
                    "confirmationRole": {
                      "thresholdFactors": [
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "device",
                              "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "0f081cd5f944efc9cae2f3262e30b445b947601b2fc668938a7c4d464c88fe69"
                                },
                                "derivationPath": {
                                  "scheme": "cap26",
                                  "path": "m/44H/1022H/1H/525H/1460H/27S"
                                }
                              }
                            }
                          }
                        }
                      ],
                      "threshold": 1,
                      "overrideFactors": [
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "device",
                              "body": "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "d3d66160cf7117b310c7875fbf8b5695ccc13116a167d13196d22dd8be18a60f"
                                },
                                "derivationPath": {
                                  "scheme": "cap26",
                                  "path": "m/44H/1022H/1H/525H/1460H/13S"
                                }
                              }
                            }
                          }
                        }
                      ]
                    }
                  }
                }
              }
            }
            "#,
        );
    }
}

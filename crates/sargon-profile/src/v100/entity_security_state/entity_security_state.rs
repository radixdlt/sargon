use crate::prelude::*;

/// Describes the state an entity - Account or Persona - is in, in regards to how
/// the user controls it, i.e. if it is controlled by a single factor (private key)
///  or an `AccessController` with a potential Multi-Factor setup.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumAsInner,
)]
#[serde(untagged, remote = "Self")]
#[allow(clippy::large_enum_variant)]
pub enum EntitySecurityState {
    /// The account is controlled by a single factor (private key)
    Unsecured {
        /// The current state of the unsecured entity
        #[serde(rename = "unsecuredEntityControl")]
        value: UnsecuredEntityControl,
    },

    /// The account is controlled by multi-factor
    Securified {
        /// The current state of the securified entity
        #[serde(rename = "securedEntityControl")]
        value: SecuredEntityControl,
    },
}

impl HasProvisionalSecurifiedConfig for EntitySecurityState {
    fn get_provisional(&self) -> Option<ProvisionalSecurifiedConfig> {
        match self {
            Self::Unsecured { value } => value.get_provisional(),
            Self::Securified { value } => value.get_provisional(),
        }
    }

    fn set_provisional_unchecked(
        &mut self,
        provisional_securified_config: impl Into<
            Option<ProvisionalSecurifiedConfig>,
        >,
    ) {
        match self {
            Self::Unsecured { value } => {
                value.set_provisional_unchecked(provisional_securified_config)
            }
            Self::Securified { value } => {
                value.set_provisional_unchecked(provisional_securified_config)
            }
        }
    }
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

impl HasFactorInstances for EntitySecurityState {
    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance> {
        match self {
            EntitySecurityState::Unsecured { value } => {
                value.unique_tx_signing_factor_instances()
            }
            EntitySecurityState::Securified { value } => {
                value.unique_tx_signing_factor_instances()
            }
        }
    }

    fn unique_all_factor_instances(&self) -> IndexSet<FactorInstance> {
        match self {
            EntitySecurityState::Unsecured { value } => {
                value.unique_all_factor_instances()
            }
            EntitySecurityState::Securified { value } => {
                value.unique_all_factor_instances()
            }
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

    fn set_provisional_is_err_when_provisional_is_tx_queued(sut: SUT) {
        let mut sut = sut;
        sut.set_provisional(ProvisionalSecurifiedConfig::TransactionQueued {
            value: ProvisionalSecurifiedTransactionQueued::sample(),
        })
        .unwrap();
        let mut clone = sut.clone();
        let mut test = |x: ProvisionalSecurifiedConfig| {
            assert!(clone.set_provisional(x).is_err());
        };

        test(ProvisionalSecurifiedConfig::ShieldSelected {
            value: SecurityStructureID::sample(),
        });
        test(ProvisionalSecurifiedConfig::FactorInstancesDerived {
            value: SecurityStructureOfFactorInstances::sample(),
        });
        test(ProvisionalSecurifiedConfig::TransactionQueued {
            value: ProvisionalSecurifiedTransactionQueued::sample_other(),
        });

        assert_eq!(clone, sut); //  unchanged
    }

    fn can_change_shield(sut: SUT) {
        let mut sut = sut;
        let id1 = ProvisionalSecurifiedConfig::ShieldSelected {
            value: SecurityStructureID::sample(),
        };
        let id2 = ProvisionalSecurifiedConfig::ShieldSelected {
            value: SecurityStructureID::sample_other(),
        };
        sut.set_provisional(id1.clone()).unwrap();
        assert_eq!(sut.get_provisional().unwrap(), id1);
        assert!(sut.set_provisional(id2.clone()).is_ok());
        assert_eq!(sut.get_provisional().unwrap(), id2);
    }

    #[test]
    fn can_change_shield_id_unsecured() {
        can_change_shield(SUT::from(UnsecuredEntityControl::sample()));
    }

    #[test]
    fn can_change_shield_id_secured() {
        can_change_shield(SUT::from(SecuredEntityControl::sample_other()));
    }

    fn set_provisional_to_non_tx_queued_is_err_when_provisional_is_instances_derived(
        sut: SUT,
    ) {
        let mut sut = sut;
        sut.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: SecurityStructureOfFactorInstances::sample(),
            },
        )
        .unwrap();
        let mut clone = sut.clone();
        let mut test = |x: Option<ProvisionalSecurifiedConfig>| {
            assert!(clone.set_provisional(x).is_err());
        };
        test(None);
        test(Some(ProvisionalSecurifiedConfig::ShieldSelected {
            value: SecurityStructureID::sample(),
        }));
        test(Some(ProvisionalSecurifiedConfig::FactorInstancesDerived {
            value: SecurityStructureOfFactorInstances::sample(),
        }));

        assert_eq!(clone, sut); //  unchanged
    }

    #[test]
    fn set_provisional_to_non_tx_queued_is_err_when_provisional_is_instances_derived_unsecure(
    ) {
        set_provisional_to_non_tx_queued_is_err_when_provisional_is_instances_derived(SUT::from(UnsecuredEntityControl::sample()));
    }

    #[test]
    fn set_provisional_to_non_tx_queued_is_err_when_provisional_is_instances_derived_secure(
    ) {
        set_provisional_to_non_tx_queued_is_err_when_provisional_is_instances_derived(SUT::from(SecuredEntityControl::sample_other()));
    }

    #[test]
    fn set_provisional_is_err_when_provisional_is_tx_queued_unsecured() {
        set_provisional_is_err_when_provisional_is_tx_queued(SUT::from(
            UnsecuredEntityControl::sample(),
        ));
    }

    #[test]
    fn set_provisional_is_err_when_provisional_is_tx_queued_secured() {
        set_provisional_is_err_when_provisional_is_tx_queued(SUT::from(
            SecuredEntityControl::sample_other(),
        ));
    }

    fn cancel_works(sut: SUT) {
        let mut sut = sut;
        let tx_queued = ProvisionalSecurifiedTransactionQueued::sample();
        sut.set_provisional(ProvisionalSecurifiedConfig::TransactionQueued {
            value: tx_queued.clone(),
        })
        .unwrap();
        assert!(sut.cancel_queued_transaction().is_ok());

        assert_eq!(
            sut.get_provisional().unwrap(),
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: tx_queued.clone().factor_instances
            }
        ); // assert is cancelled

        // can re-queue tx
        sut.set_provisional(ProvisionalSecurifiedConfig::TransactionQueued {
            value: tx_queued.clone(),
        })
        .unwrap();
    }

    #[test]
    fn cancel_works_unsecured() {
        cancel_works(SUT::from(UnsecuredEntityControl::sample()));
    }

    #[test]
    fn cancel_works_secured() {
        cancel_works(SUT::from(SecuredEntityControl::sample_other()));
    }

    fn cancel_when_no_tx_queued_is_err(val: SUT) {
        let test = |x: Option<ProvisionalSecurifiedConfig>| {
            let mut sut = val.clone();
            sut.set_provisional(x).unwrap();
            let clone = sut.clone();
            assert!(sut.cancel_queued_transaction().is_err());
            assert_eq!(clone, sut) // unchanged
        };

        test(None);
        test(Some(ProvisionalSecurifiedConfig::ShieldSelected {
            value: SecurityStructureID::sample(),
        }));
        test(Some(ProvisionalSecurifiedConfig::FactorInstancesDerived {
            value: SecurityStructureOfFactorInstances::sample(),
        }));
    }

    #[test]
    fn cancel_when_no_tx_queued_is_err_unsecured() {
        cancel_when_no_tx_queued_is_err(SUT::from(
            UnsecuredEntityControl::sample(),
        ))
    }

    #[test]
    fn cancel_when_no_tx_queued_is_err_secured() {
        cancel_when_no_tx_queued_is_err(SUT::from(
            SecuredEntityControl::sample_other(),
        ))
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
                "veci": {
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
                          "compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
                        },
                        "derivationPath": {
                          "scheme": "cap26",
                          "path": "m/44H/1022H/1H/525H/1460H/0H"
                        }
                      }
                    }
                  }
                },
                "accessControllerAddress": "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a",
                "securityStructure": {
                  "securityStructureId": "ffffffff-ffff-ffff-ffff-ffffffffffff",
                  "matrixOfFactors": {
                    "primaryRole": {
                      "threshold": 2,
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
                        },
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "ledgerHQHardwareWallet",
                              "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "92cd6838cd4e7b0523ed93d498e093f71139ffd5d632578189b39a26005be56b"
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
                      "overrideFactors": []
                    },
                    "recoveryRole": {
                      "threshold": 0,
                      "thresholdFactors": [],
                      "overrideFactors": [
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
                        },
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "ledgerHQHardwareWallet",
                              "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "92cd6838cd4e7b0523ed93d498e093f71139ffd5d632578189b39a26005be56b"
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
                    "confirmationRole": {
                      "threshold": 0,
                      "thresholdFactors": [],
                      "overrideFactors": [
                        {
                          "factorSourceID": {
                            "discriminator": "fromHash",
                            "fromHash": {
                              "kind": "password",
                              "body": "181ab662e19fac3ad9f08d5c673b286d4a5ed9cd3762356dc9831dc42427c1b9"
                            }
                          },
                          "badge": {
                            "discriminator": "virtualSource",
                            "virtualSource": {
                              "discriminator": "hierarchicalDeterministicPublicKey",
                              "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                  "curve": "curve25519",
                                  "compressedData": "4af49eb56b1af579aaf03f1760ec526f56e2297651f7a067f4b362f685417a81"
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
                    "numberOfDaysUntilAutoConfirm": 14
                  },
                  "authenticationSigningFactorInstance": {
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
                            "compressedData": "136b3a73595315517f921767bc49ae3ba43fc25d2e34e51fbff434a329176ee8"
                          },
                          "derivationPath": {
                            "scheme": "cap26",
                            "path": "m/44H/1022H/1H/525H/1678H/0S"
                          }
                        }
                      }
                    }
                  }
                },
                "provisionalSecurifiedConfig": {
                  "discriminator": "factorInstancesDerived",
                  "value": {
                    "securityStructureId": "dededede-dede-dede-dede-dededededede",
                    "matrixOfFactors": {
                      "primaryRole": {
                        "threshold": 1,
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
                                    "compressedData": "a40a1850ade79f5b24956b4abdb94624ba8189f68ad39fd2bb92ecdc2cbe17d2"
                                  },
                                  "derivationPath": {
                                    "scheme": "cap26",
                                    "path": "m/44H/1022H/1H/618H/1460H/0S"
                                  }
                                }
                              }
                            }
                          }
                        ],
                        "overrideFactors": []
                      },
                      "recoveryRole": {
                        "threshold": 0,
                        "thresholdFactors": [],
                        "overrideFactors": [
                          {
                            "factorSourceID": {
                              "discriminator": "fromHash",
                              "fromHash": {
                                "kind": "ledgerHQHardwareWallet",
                                "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                              }
                            },
                            "badge": {
                              "discriminator": "virtualSource",
                              "virtualSource": {
                                "discriminator": "hierarchicalDeterministicPublicKey",
                                "hierarchicalDeterministicPublicKey": {
                                  "publicKey": {
                                    "curve": "curve25519",
                                    "compressedData": "6f7ac7d9031e321d1762431941b672f164ebb5a6dd2ded9b0c8da2b278143c74"
                                  },
                                  "derivationPath": {
                                    "scheme": "cap26",
                                    "path": "m/44H/1022H/1H/618H/1460H/0S"
                                  }
                                }
                              }
                            }
                          }
                        ]
                      },
                      "confirmationRole": {
                        "threshold": 0,
                        "thresholdFactors": [],
                        "overrideFactors": [
                          {
                            "factorSourceID": {
                              "discriminator": "fromHash",
                              "fromHash": {
                                "kind": "ledgerHQHardwareWallet",
                                "body": "52ef052a0642a94279b296d6b3b17dedc035a7ae37b76c1d60f11f2725100077"
                              }
                            },
                            "badge": {
                              "discriminator": "virtualSource",
                              "virtualSource": {
                                "discriminator": "hierarchicalDeterministicPublicKey",
                                "hierarchicalDeterministicPublicKey": {
                                  "publicKey": {
                                    "curve": "curve25519",
                                    "compressedData": "e867cd64b70cccad642f47ee4acff014b982870cf5218fbd56da79b0eb6e9fba"
                                  },
                                  "derivationPath": {
                                    "scheme": "cap26",
                                    "path": "m/44H/1022H/1H/618H/1460H/0S"
                                  }
                                }
                              }
                            }
                          }
                        ]
                      },
                      "numberOfDaysUntilAutoConfirm": 14
                    },
                    "authenticationSigningFactorInstance": {
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
                              "compressedData": "d2343d84e7970224ad4f605782f78b096b750f03990c927492ba5308258c689a"
                            },
                            "derivationPath": {
                              "scheme": "cap26",
                              "path": "m/44H/1022H/1H/618H/1678H/0S"
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
            "#,
        );
    }

    #[test]
    fn unique_tx_signing_factor_instances() {
        let unsecured = UnsecuredEntityControl::sample();
        let sut = SUT::Unsecured {
            value: unsecured.clone(),
        };
        assert_eq!(
            sut.unique_tx_signing_factor_instances(),
            unsecured.unique_tx_signing_factor_instances()
        );

        let secured = SecuredEntityControl::sample();
        let sut = SUT::Securified {
            value: secured.clone(),
        };
        assert_eq!(
            sut.unique_tx_signing_factor_instances(),
            secured.unique_tx_signing_factor_instances()
        );
    }
}

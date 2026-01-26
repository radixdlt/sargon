use crate::prelude::*;

/// A structure of factors to use for certain roles, Primary, Recovery and
/// Confirmation, as well as an authentication signing factor instance which is
/// used for Rola.
///
/// This structure is identified by the `security_structure_id` which is the ID
/// of the `SecurityStructureOfFactorSourceIDs` which was used to derive the
/// instances in this structure.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStructureOfFactorInstances {
    /// The ID of the `SecurityStructureOfFactorSourceIDs` in
    /// `profile.app_preferences.security.security_structures_of_factor_source_ids`
    /// which was used to derive the factor instances in this structure. Or rather:
    /// The id of `SecurityStructureOfFactorSources`.
    pub security_structure_id: SecurityStructureID,

    /// The structure of factors to use for certain roles, Primary, Recovery
    /// and Confirmation role.
    pub matrix_of_factors: MatrixOfFactorInstances,

    /// The authentication signing factor instance which is used to sign
    /// proof of ownership - aka "True Rola Key". User can select which FactorSource
    /// to use during Shield Building, but typically most users will use the
    /// DeviceFactorSource which is default. DerivationPath is in securified
    /// KeySpace of course.
    ///
    /// Non-optional since we can replace it with a new one for entities
    /// we have recovered during Onboarding Account Recovery Scan for securified
    /// entities
    pub authentication_signing_factor_instance:
        HierarchicalDeterministicFactorInstance,
}

impl HasFactorInstances for SecurityStructureOfFactorInstances {
    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.matrix_of_factors.unique_tx_signing_factor_instances()
    }

    fn unique_all_factor_instances(&self) -> IndexSet<FactorInstance> {
        let mut instances = self.unique_tx_signing_factor_instances();
        instances
            .insert(self.authentication_signing_factor_instance.clone().into());
        instances
    }
}

impl SecurityStructureOfFactorInstances {
    pub fn new(
        security_structure_id: SecurityStructureID,
        matrix_of_factors: MatrixOfFactorInstances,
        authentication_signing: HierarchicalDeterministicFactorInstance,
    ) -> Result<Self> {
        let index_agnostic_path = matrix_of_factors
            .index_agnostic_path_of_all_tx_signing_factor_instances()?;

        if authentication_signing.get_key_kind()
            != CAP26KeyKind::AuthenticationSigning
        {
            return Err(
                CommonError::WrongKeyKindOfAuthenticationSigningFactorInstance,
            );
        }

        if authentication_signing.get_entity_kind()
            != index_agnostic_path.entity_kind
        {
            return Err(CommonError::WrongEntityKindOfInFactorInstancesPath);
        }

        if !authentication_signing.is_securified() {
            return Err(
                CommonError::AuthenticationSigningFactorInstanceNotSecurified,
            );
        }

        Ok(Self {
            security_structure_id,
            matrix_of_factors,
            authentication_signing_factor_instance: authentication_signing,
        })
    }
}

impl SecurityStructureOfFactorInstances {
    pub fn timed_recovery_delay_in_minutes(&self) -> u32 {
        self.matrix_of_factors.timed_recovery_delay_in_minutes()
    }
}

impl Identifiable for SecurityStructureOfFactorInstances {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.security_structure_id
    }
}

impl HasSampleValues for SecurityStructureOfFactorInstances {
    /// Account
    fn sample() -> Self {
        Self::new(
            SecurityStructureID::sample(),
            MatrixOfFactorInstances::sample(),
            HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(NetworkID::Mainnet, CAP26KeyKind::AuthenticationSigning, CAP26EntityKind::Account, Hardened::Securified(SecurifiedU30::ZERO))
        )
        .unwrap()
    }

    /// Persona
    fn sample_other() -> Self {
        Self::new(
            SecurityStructureID::sample_other(),
            MatrixOfFactorInstances::sample_other(),
            HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(NetworkID::Mainnet, CAP26KeyKind::AuthenticationSigning, CAP26EntityKind::Identity, Hardened::Securified(SecurifiedU30::ZERO))
        )
        .unwrap()
    }
}

impl SecurityStructureOfFactorInstances {
    pub fn sample_sim() -> Self {
        Self::new(
      SecurityStructureID::sample(),
      MatrixOfFactorInstances::sample_sim(),
      HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(NetworkID::Simulator, CAP26KeyKind::AuthenticationSigning, CAP26EntityKind::Account, Hardened::Securified(SecurifiedU30::ZERO))
  )
  .unwrap()
    }

    pub fn sample_other_sim() -> Self {
        Self::new(
      SecurityStructureID::sample(),
      MatrixOfFactorInstances::sample_other_sim(),
      HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(NetworkID::Simulator, CAP26KeyKind::AuthenticationSigning, CAP26EntityKind::Account, Hardened::Securified(SecurifiedU30::ZERO))
  )
  .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureOfFactorInstances;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn unique_all_factor_instances() {
        let sut = SUT::sample();
        assert!(sut
            .unique_all_factor_instances()
            .into_iter()
            .map(|f| f.try_as_hd_factor_instances().unwrap())
            .any(|f| f.derivation_path().get_key_kind()
                == CAP26KeyKind::AuthenticationSigning));
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn timed_recovery_delay_in_minutes() {
        let sut = SUT::sample();
        assert_eq!(sut.timed_recovery_delay_in_minutes(), 20160);
    }

    #[test]
    fn wrong_entity_kind_of_auth_signing_factor() {
        let res = SUT::new(SecurityStructureID::sample(), MatrixOfFactorInstances::sample(), HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(NetworkID::Mainnet, CAP26KeyKind::AuthenticationSigning, CAP26EntityKind::Identity, Hardened::Securified(SecurifiedU30::ZERO)));
        assert!(matches!(
            res,
            Err(CommonError::WrongEntityKindOfInFactorInstancesPath)
        ));
    }

    #[test]
    fn id() {
        assert_eq!(SUT::sample().id(), SUT::sample().security_structure_id);
    }

    #[test]
    fn wrong_key_kind_of_auth_signing_factor() {
        let res = SUT::new(SecurityStructureID::sample(), MatrixOfFactorInstances::sample(), HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, CAP26EntityKind::Account, Hardened::Securified(SecurifiedU30::ZERO)));
        assert!(matches!(
            res,
            Err(CommonError::WrongKeyKindOfAuthenticationSigningFactorInstance)
        ));
    }

    #[test]
    fn auth_signing_factor_not_signing() {
        let res = SUT::new(SecurityStructureID::sample(), MatrixOfFactorInstances::sample(), HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(NetworkID::Mainnet, CAP26KeyKind::AuthenticationSigning, CAP26EntityKind::Account, Hardened::Unsecurified(UnsecurifiedHardened::ZERO)));
        assert!(matches!(
            res,
            Err(CommonError::AuthenticationSigningFactorInstanceNotSecurified)
        ));
    }

    #[test]
    fn json_roundtrip_sample_other() {
        let sut = SUT::sample_other();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "securityStructureId": "dededede-dede-dede-dede-dededededede",
              "matrixOfFactors": {
                "primaryRole": {
                  "threshold": "all",
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
                  "threshold": "all",
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
                  "threshold": "all",
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
                "timeUntilDelayedConfirmationIsCallable": {
                	"value": 2,
                	"unit": "weeks"
                }
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
        "#,
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "securityStructureId": "ffffffff-ffff-ffff-ffff-ffffffffffff",
              "matrixOfFactors": {
                "primaryRole": {
                  "threshold": "all",
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
                  "threshold": "all",
                  "thresholdFactors": [],
                  "overrideFactors": [
                    {
                      "factorSourceID": {
                        "discriminator": "fromHash",
                        "fromHash": {
                          "kind": "arculusCard",
                          "body": "12f36554769cd96614776e6dbd5629825b8e87366eec5e515de32bb1ea153820"
                        }
                      },
                      "badge": {
                        "discriminator": "virtualSource",
                        "virtualSource": {
                          "discriminator": "hierarchicalDeterministicPublicKey",
                          "hierarchicalDeterministicPublicKey": {
                            "publicKey": {
                              "curve": "curve25519",
                              "compressedData": "999bc2b17d012c3ce49da85b880029be5f9a9093247821f746ba73b6fc20e406"
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
                  "threshold": "all",
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
                "timeUntilDelayedConfirmationIsCallable": {
                	"value": 2,
                	"unit": "weeks"
                }
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
            }
        "#,
        );
    }
}
